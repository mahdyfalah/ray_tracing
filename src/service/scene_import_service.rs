use rfd::FileDialog;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug)]
pub struct SceneImportService;

impl SceneImportService {
    // /// Opens a file dialog to select an XML file and parses it into a Scene object.
    // pub fn import_scene() -> Option<Scene> {
    //     // Open file dialog
    //     let file = FileDialog::new()
    //         .add_filter("XML files", &["xml"])
    //         .pick_file();
    //
    //     match file {
    //         Some(path) => {
    //             println!("Selected file: {:?}", path);
    //             // Attempt to read and parse the file
    //             match Self::parse_scene_from_file(&path) {
    //                 Ok(scene) => Some(scene),
    //                 Err(err) => {
    //                     eprintln!("Error parsing scene: {:?}", err);
    //                     None
    //                 }
    //             }
    //         }
    //         None => {
    //             println!("No file selected");
    //             None
    //         }
    //     }
    // }
    //
    // /// Parses a Scene object from the provided file path
    // fn parse_scene_from_file(path: &std::path::Path) -> Result<Scene, Box<dyn std::error::Error>> {
    //     let mut file = File::open(path)?;
    //     let mut content = String::new();
    //     file.read_to_string(&mut content)?;
    //
    //     // Deserialize just the background color for now
    //     let scene: Scene = serde_xml_rs::from_str(&content)?;
    //
    //     // Here you would set a default camera if needed, for now we set it to None
    //     Ok(scene)
    // }
}
