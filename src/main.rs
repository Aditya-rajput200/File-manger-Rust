use std::{fs, path};
use std::path::Path;

use chrono::Local;
use colored::*;
use dialoguer::theme;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

// #[derive(Parser, Debug)]
// #[command(
//     name = "File Organizer",
//     author = "Aditya Raj",
//     version = "3.0",
//     about = "Organizes files BY size and types in Supersonic speed."
// )]
// struct Args {
//     /// Folder path to organize
//     #[arg(short, long)]
//     path: String,

//     /// Perform a dry run (no actual moves)
//     #[arg(short, long, default_value_t = false)]
//     dry_run: bool,

//     /// Recurse into subdirectories
//     #[arg(short, long, default_value_t = false)]
//     recursive: bool,
// }

//detect the file type and seprate them
fn file_type(exe: &str) -> &str {
    match exe {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => "Images",
        "mp4" | "mkv" | "mov" | "avi" => "Videos",
         "mp3" | "wav" | "flac" => "Music",
        "pdf" | "doc" | "docx" | "txt" => "Documents",
        "zip" | "rar" | "7z" => "Archives",
        "exe" | "msi" => "Applications",
        _ => "Others",
    }
}
fn file_size_type(size:f64) -> &'static str {
   
    const MB:f64 = 1024.0* 1024.0;

    // converting bytes into mb
    let byte_mb =size/MB; 
     if byte_mb<400.0 {
        return  "Small_Files";
        
     }
     else if byte_mb >=400.0 && byte_mb<1000.0 {
        return  "Medium_Files";
         
     }
     else {
         return  "Large_Files";
     }
}
fn size_sepration (folder_path: &str) {


 let entries:Vec<_> = fs::read_dir(folder_path)
 .unwrap()
 .filter_map(|e| e.ok())
 .collect();;
     let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} files processed")
            .unwrap()
            .progress_chars("█▓░"),
    );

 for  entry in entries {

    let path = entry.path();

    // now get the file size 
    if path.is_file() {
    let metadata = fs::metadata(&path).unwrap();
    let file_size = metadata.len();

    // get the file name acc to file size 
    let category =  file_size_type(file_size as f64);

    // make the folders 
    let new_folder = Path::new(folder_path).join(category);
    fs::create_dir_all(&new_folder);
                 let file_name = path.file_name().unwrap();
                let new_path = new_folder.join(file_name);

                // move the file
                fs::rename(&path, &new_path).unwrap();

            
    
    

    }
     bar.inc(1);
 }
 bar.finish_with_message("✅ Size-based separation complete!");
 println!("SuccesFully completd File Size Based sepration ✅✅✅")

}

fn types_sepration (folder_path: &str ) {

   // let entries = fs::read_dir(folder_path).unwrap();
    let entries :Vec<_> = WalkDir::new(folder_path)
    .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

        let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} files processed")
            .unwrap()
            .progress_chars("█▓░"),
    );

       for entry in entries {
      
        let path = entry.path();

        //skip the folders
        if path.is_file() {
            // get the extensions
            if let Some(ext) = path.extension() {
                //extract the extension string
                let ext_name = ext.to_string_lossy().to_lowercase();

                let category = file_type(&ext_name);

                // creating the folder
                let new_folders = Path::new(folder_path).join(category);
                fs::create_dir_all(&new_folders).unwrap();

                // now seprate the file types into created folder
                let file_name = path.file_name().unwrap();
                let target_path = new_folders.join(file_name);

                // move the file
                fs::rename(&path, &target_path).unwrap();

              
            }
        }
         bar.inc(1);
    }
     bar.finish_with_message("✅File Type-based separation completed!");
     println!("SuccesFully completd File type Based sepration ✅✅✅")
}



fn main() {


let options =vec!["Type based sepration","Size Based sepration","Exit"];

 let folder_path:String = Input::new().with_prompt("Provide file path").interact_text().unwrap();

 let selection = Select::with_theme(&theme::ColorfulTheme::default())
  .with_prompt("Choose an action")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

        match selection {
            0 =>types_sepration(&folder_path),
            1 => size_sepration(&folder_path), 
            3 => println!("Good Bye.."),
            _ => println!("Some thing went worng"),
}

 
}
