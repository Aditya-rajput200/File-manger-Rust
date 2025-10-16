use std::{fs, path};
use std::path::Path;
//detect the file type and seprate them
fn file_type(exe: &str) -> &str {
    match exe {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => "Images",
        "mp4" | "mkv" | "mov" | "avi" => "Videos",
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
 let entries = fs::read_dir(folder_path).unwrap();

 for  entry in entries {
    let entry  = entry.unwrap();
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

                println!(
                    "Moved: {:?} → {:?}",
                    path.file_name().unwrap(),
                    new_path.display()
                );
    
    

    }
    
 }


}

fn types_sepration (folder_path: &str ) {
    let entries = fs::read_dir(folder_path).unwrap();
       for entry in entries {
        let entry = entry.unwrap();
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

                println!(
                    "Moved: {:?} → {:?}",
                    path.file_name().unwrap(),
                    target_path.display()
                );
            }
        }
    }
}
fn main() {
    let folder_path: &'static str = "E:/test";
   types_sepration(folder_path);

   size_sepration(folder_path);
   

 
}
