use crate::models::{point, vector};

pub mod scene_import_service;
pub mod render_service;
pub mod obj_parser_service;

pub type Vertex = point::Point;
pub type Normal = vector::Vector;