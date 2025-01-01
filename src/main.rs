use ray_tracing::service::scene_import_service::SceneImportService;

fn main() {
    match SceneImportService::import_scene() {
        Ok(scene) => println!("Scene imported successfully: {:?}", scene),
        Err(err) => eprintln!("Error: {}", err),
    }
}
