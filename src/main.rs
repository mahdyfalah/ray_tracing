use ray_tracing::services::render_service::RenderService;
use ray_tracing::services::scene_import_service::SceneImportService;
// use ray_tracing::services::obj_parser_service::{ObjModel, read_obj_file};

fn main() {
    match SceneImportService::import_scene() {
        Ok(scene) => {
            RenderService::generate_image(&scene);
        }
        Err(err) => {
            eprintln!("Failed to load scene: {}", err);
        }
    }

    // let model = read_obj_file("example-input/open_room.obj").unwrap();
    // println!("{:#?}", model);
}
