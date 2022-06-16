// use std::{error::Error, fs::read_dir, path::Path};

// pub fn sync() -> Result<(), Box<dyn Error>> {
//     let home_path = home_dir().ok_or("Could not find home folder")?;
//     let repository_path = config_dir()
//         .ok_or("Could not find config directory")?
//         .join("dimport/.repository");

//     if !repository_path.exists() {
//         return Err("Please initialize first".into());
//     }

//     traverse_files(&repository_path, |file| println!("{:?}", file))?;

//     Ok(())
// }

// fn traverse_files(dir: &Path, op: fn(&Path) -> ()) -> Result<(), Box<dyn Error>> {
//     for entry in read_dir(dir)? {
//         let path = entry?.path();
//         if path.is_dir() {
//             traverse_files(&path, op)?;
//         } else {
//             op(&path)
//         }
//     }

//     Ok(())
// }
