use rfd::FileDialog;
use std::fs::File;
use std::io::{Read};
use std::path::Path;
use serde_xml_rs::from_str;
use crate::models::scene::Scene;

#[derive(Debug)]
pub struct SceneImportService;

impl SceneImportService {
    /// Opens a file dialog to select an XML file and parses it into a Scene object.
    pub fn import_scene() -> Result<Scene, String> {
        let file = FileDialog::new()
            .add_filter("XML files", &["xml"])
            .pick_file();

        match file {
            Some(path) => {
                println!("Selected file: {:?}", path);

                match Self::parse_scene_from_file(&path) {
                    Ok(scene) => Ok(scene),
                    Err(err) => Err(format!("Failed to parse scene: {}", err)),
                }
            }
            None => Err(String::from("No file selected")),
        }
    }

    /// Parses a Scene object from the provided file path
    /// used library: serde-xml-rs
    fn parse_scene_from_file(path: &Path) -> Result<Scene, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut scene: Scene = from_str(&content)?;
        scene.load_meshes()?;

        println!("Selected scene: {:?}", scene);

        Ok(scene)
    }
}
