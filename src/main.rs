
use  std::fs;   
fn main () { 
    
    let folder_path ="C:/Users/DELL/Documents";

    let entries = fs::read_dir(folder_path).unwrap();

    for entry in entries {
         let entry = entry.unwrap();
        println!("Found: {}",entry.path().display());
    }

}