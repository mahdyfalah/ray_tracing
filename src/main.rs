use ray_tracing::services::image_generate_service::RenderService;
use ray_tracing::services::scene_import_service::SceneImportService;

fn main() {
    match SceneImportService::import_scene() {
        Ok(scene) => {
            RenderService::generate_image(&scene);
        }
        Err(err) => {
            eprintln!("Failed to load scene: {}", err);
        }
    }
}
