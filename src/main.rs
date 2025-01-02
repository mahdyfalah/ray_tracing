use ray_tracing::services::image_generate_service::ImageGenerateService;
use ray_tracing::services::scene_import_service::SceneImportService;

fn main() {
    match SceneImportService::import_scene() {
        Ok(scene) => {
            ImageGenerateService::generate_image(&scene);
        }
        Err(err) => {
            eprintln!("Failed to load scene: {}", err);
        }
    }
}
