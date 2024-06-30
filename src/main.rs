//This program is meant to scan for all files in a folder including subfolders and move all files to one folder, thus destroying the original file structure.
//Usage [options] "source" "destination".
//Options --help to print help;;; --no-rewrite. to append numbers to files with similar names
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use fs_extra::file::{move_file, CopyOptions};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("File Structure Destroyer")
        .version("1.0")
        .author("..")
        .about("This program is meant to scan for all files in a folder including subfolders and move all files to one folder, thus destroying the original file structure.")
        .arg(Arg::new("source")
            .help("The source directory to scan")
            .required(true)
            .index(1))
        .arg(Arg::new("destination")
            .help("The destination directory")
            .required(true)
            .index(2))
        .arg(Arg::new("--no-rewrite")
            .help("Appends numbers to filenames if they already exist in the destination")
            .long("no-rewrite"))
        .get_matches();

    let source_dir:String;
    match  matches.get_one::<& str>("source") {
        Some(source)=>{source_dir = source.to_string()}, 
        None =>{
            println!("No source specified");
            return; //Error handling
        }
    }
    let dest_dir:String  ;
    match matches.get_one::<& str>("destination") {
        Some(dest)=>{dest_dir = dest.to_string()}, 
        None =>{
            println!("No destination specified");
            return; //Error handling
        }
    }
    let no_rewrite = matches.get_flag("--no-rewrite");
    let dest_dir_copy = dest_dir.as_str();
    // Create destination directory if it doesn't exist
    if !Path::new(dest_dir_copy).exists() {
        match fs::create_dir_all(dest_dir_copy) {
            Ok(_)=>{println!("Created destination directory")}
            Err(err)=>{println!("{err}")}
        }
    }

    // Iterate over all files in the source directory and its subdirectories
    for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().expect("Failed to get file name");
            let mut dest_path = PathBuf::from(dest_dir.clone()).join(file_name);
            
            //if no reqrite was specified then get a unique numbered file name
            if no_rewrite {
                dest_path = get_unique_file_path(&dest_path);
            }

            if dest_path.exists() && !no_rewrite {
                println!("File {:?} already exists and --no-rewrite not specified. Skipping.", dest_path);
                continue;
            }

            let options = CopyOptions::new();
            //move the file and handle errors
            match move_file(path, &dest_path, &options) {
                Ok(_)=>{println!("Moved: {:?}",file_name)}
                Err(err)=>{println!("{err}");}              
            }
        }
    }

    // Optionally, you can remove the original directory structure
    //fs::remove_dir_all(source_dir).expect("Failed to remove source directory");
}

fn get_unique_file_path(mut path: &PathBuf) -> PathBuf {
    let mut counter = 1;
    let mut new_path = path.clone();

    while new_path.exists() {
        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let new_file_name = format!("{}-{}.{}", file_stem, counter, extension);
        new_path = path.with_file_name(new_file_name);
        counter += 1;
    }

    new_path
}

