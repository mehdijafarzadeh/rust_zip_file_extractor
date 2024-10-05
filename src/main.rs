use std::fs;    // Import the file system operations module.
use std::io;    // Import general input/output functions.

fn main() {
    // The main function exits with the return code of `real_main()`.
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    // Collect all command-line arguments into a vector.
    let args: Vec<_> = std::env::args().collect();

    // If there are fewer than 2 arguments, print a usage message and return an error code (1).
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);  // args[0] is the program's name.
        return 1;
    }

    // Get the path to the ZIP file from the second command-line argument.
    let fname = std::path::Path::new(&*args[1]);

    // Open the ZIP file. `unwrap()` is used to stop the program if opening the file fails.
    let file = fs::File::open(&fname).unwrap();

    // Create a ZIP archive object from the file using `ZipArchive`.
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // Loop over all files inside the ZIP archive.
    for i in 0..archive.len() {
        // Get each file in the ZIP archive by its index.
        let mut file = archive.by_index(i).unwrap();

        // Ensure the file name doesn't contain directory traversal paths like `../`.
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),  // Get the safe path as `outpath`.
            None => continue,  // Skip this file if the name is not valid.
        };

        // Print any comment that the file might have (some ZIP files include optional comments).
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {}, comment: {}", i, comment);  // Print file index and its comment.
            }
        }

        // Check if the current file is a directory (its name ends with '/').
        if (*file.name()).ends_with('/') {
            // Print that a directory has been extracted.
            println!("File {} extracted to \"{}\"", i, outpath.display());

            // Create the directory, including any necessary parent directories.
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // Print that a file has been extracted, along with its size.
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(), file.size());

            // Ensure the parent directory for the file exists.
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();  // Create the parent directories if needed.
                }
            }

            // Create the extracted file at `outpath`.
            let mut outfile = fs::File::create(&outpath).unwrap();

            // Copy the file's contents from the ZIP archive to the newly created file.
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Unix-specific permissions handling (only runs on Unix systems).
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;  // Import Unix-specific permission handling.
            if let Some(mode) = file.unix_mode() {
                // Set the file's permissions to match the original permissions stored in the ZIP file.
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // Return 0 to indicate success.
    0
}