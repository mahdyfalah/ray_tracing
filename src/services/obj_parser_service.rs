use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::models::{Vertex, Normal};

#[derive(Debug)]
pub struct Face {
    vertex_indices: Vec<usize>,
    texture_indices: Vec<usize>,
    normal_indices: Vec<usize>,
}

#[derive(Debug)]
pub struct ObjModel {
    vertices: Vec<Vertex>,
    texture_coords: Vec<(f64, f64)>,
    normals: Vec<Normal>,
    faces: Vec<Face>,
}

impl ObjModel {
    fn new() -> Self {
        ObjModel {
            vertices: Vec::new(),
            texture_coords: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        }
    }

    fn parse_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "v" => self.parse_vertex(&parts),
            "vt" => self.parse_texture_coord(&parts),
            "vn" => self.parse_normal(&parts),
            "f" => self.parse_face(&parts),
            _ => {} // Ignore comments and unsupported lines
        }
    }

    fn parse_vertex(&mut self, parts: &[&str]) {
        if parts.len() >= 4 {
            let x = parts[1].parse().unwrap();
            let y = parts[2].parse().unwrap();
            let z = parts[3].parse().unwrap();
            self.vertices.push(Vertex::new(x, y, z));
        }
    }

    fn parse_texture_coord(&mut self, parts: &[&str]) {
        if parts.len() >= 3 {
            let u = parts[1].parse().unwrap();
            let v = parts[2].parse().unwrap();
            self.texture_coords.push((u, v));
        }
    }

    fn parse_normal(&mut self, parts: &[&str]) {
        if parts.len() >= 4 {
            let x = parts[1].parse().unwrap();
            let y = parts[2].parse().unwrap();
            let z = parts[3].parse().unwrap();
            self.normals.push(Normal::new(x, y, z));
        }
    }

    fn parse_face(&mut self, parts: &[&str]) {
        let mut vertex_indices = Vec::new();
        let mut texture_indices = Vec::new();
        let mut normal_indices = Vec::new();

        for part in parts.iter().skip(1) {
            let indices: Vec<&str> = part.split('/').collect();
            if !indices.is_empty() {
                vertex_indices.push(indices[0].parse::<usize>().unwrap() - 1); // Convert to 0-based index
            }
            if indices.len() > 1 && !indices[1].is_empty() {
                texture_indices.push(indices[1].parse::<usize>().unwrap() - 1);
            }
            if indices.len() > 2 && !indices[2].is_empty() {
                normal_indices.push(indices[2].parse::<usize>().unwrap() - 1);
            }
        }

        self.faces.push(Face {
            vertex_indices,
            texture_indices,
            normal_indices,
        });
    }
}

pub fn read_obj_file<P: AsRef<Path>>(path: P) -> io::Result<ObjModel> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut model = ObjModel::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            model.parse_line(&line);
        }
    }

    Ok(model)
}