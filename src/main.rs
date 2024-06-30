//This program is meant to scan for all files in a folder including subfolders and move all files to one folder, thus destroying the original file structure.
//Usage [options] "source" "destination".
//Options --help to print help;;; --no-rewrite. to append numbers to files with similar names
// the --copy option is used when the files are to be copied instead of moved.
//Note: The original folder is not deleted and has to be manually removed
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use fs_extra::file::{move_file, CopyOptions,copy};
use clap::{Arg, ArgAction, Command};
use indicatif::{ProgressBar, ProgressStyle};
use chrono::Duration;
use std::time::Instant;

fn main() {
    let matches = Command::new("File Structure Destroyer")
        .version("1.0")
        .author("..")
        .about("This program is meant to scan for all files in a folder including subfolders and move/copy all files to one folder, thus destroying the original file structure.")
        .arg(Arg::new("source")
            .help("The source directory to scan")
            .required(true)
            .index(1))
        .arg(Arg::new("destination")
            .help("The destination directory")
            .required(true)
            .index(2))
        .arg(Arg::new("no-rewrite")
            .help("Appends numbers to filenames if they already exist in the destination")
            .action(ArgAction::SetTrue)            
            .long("no-rewrite"))
        .arg(Arg::new("copy")
            .help("Copy files instead of moving them")
            .action(ArgAction::SetTrue)
            .long("copy"))            
        .get_matches();

        //match arguements
    let source_dir:String;
    match  matches.get_one::<String>("source") {
        Some(source)=>{source_dir = source.to_string()}, 
        None =>{
            println!("No source specified");
            return; //Error handling
        }
    }
    let dest_dir:String  ;
    match matches.get_one::<String>("destination") {
        Some(dest)=>{dest_dir = dest.to_string()}, 
        None =>{
            println!("No destination specified");
            return; //Error handling
        }
    }
    let no_rewrite = matches.get_flag("no-rewrite");
    let copy_files = matches.get_flag("copy");
    let dest_dir_copy = dest_dir.as_str();

    // Create destination directory if it doesn't exist
    if !Path::new(dest_dir_copy).exists() {
        match fs::create_dir_all(dest_dir_copy) {
            Ok(_)=>{println!("Created destination directory")}
            Err(err)=>{println!("{err}")}
        }
    }

    let start_time = Instant::now(); // start counting time
    let mut file_count = 0; // to count the number of files transferred.

     // Collect all files to move/copy
     let files: Vec<_> = WalkDir::new(source_dir)
     .into_iter()
     .filter_map(|e| e.ok())
     .filter(|e| e.path().is_file())
     .collect();

    let total_files = files.len();
    println!("Total files: {}", total_files);

    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
        .progress_chars("#>-"));

    // Iterate over all files in the source directory and its subdirectories
    for entry in files {
        let path = entry.path();
        let file_name = path.file_name().expect("Failed to get file name");
        let mut dest_path = PathBuf::from(dest_dir.clone()).join(file_name);

        if no_rewrite {
            dest_path = get_unique_file_path(&dest_path);
        }

        if dest_path.exists() && !no_rewrite {
            eprintln!("File {:?} already exists and --no-rewrite not specified. Skipping.", dest_path);
            continue;
        }

        let options = CopyOptions::new();
        if copy_files {
            copy(path, &dest_path, &options).expect("Failed to copy file");
        } else {
            move_file(path, &dest_path, &options).expect("Failed to move file");
        }

        pb.set_message(format!("Processing file: {:?}", file_name));
        pb.inc(1);
        file_count += 1;
    }

    pb.finish_with_message("Operation complete");
    let duration = Instant::now().duration_since(start_time);
    let duration = Duration::from_std(duration).unwrap();

    println!("Processed {} files in {:?}", file_count, duration);

    // Optionally, you can remove the original directory structure if moving files
    /*if !copy_files {
        fs::remove_dir_all(source_dir).expect("Failed to remove source directory");
    }
    }*/

    fn get_unique_file_path(path: &PathBuf) -> PathBuf {
    let mut counter = 1;
    let mut new_path = path.clone();

    while new_path.exists() {
        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let new_file_name = if extension.is_empty() {
            format!("{}-{}", file_stem, counter)
        } else {
            format!("{}-{}.{}", file_stem, counter, extension)
        };
        new_path = path.with_file_name(new_file_name);
        counter += 1;
    }

    new_path
    }
}