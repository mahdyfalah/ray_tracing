use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::models::{Vertex, Normal};
use crate::models::triangle::Triangle;

#[derive(Debug)]
pub struct Face {
    pub vertex_indices: [usize; 3],  // Indices for 3 vertices
    pub texture_indices: [usize; 3], // Indices for 3 texture coordinates
    pub normal_indices: [usize; 3],  // Indices for 3 normals
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
        // Ensure the face has exactly 3 vertices (a triangle)
        if parts.len() != 4 {
            eprintln!("Only triangular faces are supported. Skipping face: {:?}", parts);
            return;
        }

        let mut vertex_indices = [0; 3];
        let mut texture_indices = [0; 3];
        let mut normal_indices = [0; 3];

        for (i, part) in parts.iter().skip(1).enumerate() {
            let indices: Vec<&str> = part.split('/').collect();

            vertex_indices[i] = indices[0].parse::<usize>().unwrap() - 1; // Convert to 0-based index

            if indices.len() > 1 && !indices[1].is_empty() {
                texture_indices[i] = indices[1].parse::<usize>().unwrap() - 1;
            }

            if indices.len() > 2 && !indices[2].is_empty() {
                normal_indices[i] = indices[2].parse::<usize>().unwrap() - 1;
            }
        }

        self.faces.push(Face {
            vertex_indices,
            texture_indices,
            normal_indices,
        });
    }

    /// Converts the OBJ model into a list of triangles
    pub fn to_triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        for face in &self.faces {
            // Get vertices
            let v0 = self.vertices[face.vertex_indices[0]];
            let v1 = self.vertices[face.vertex_indices[1]];
            let v2 = self.vertices[face.vertex_indices[2]];

            // Get normals (if available)
            let normal = if !self.normals.is_empty() {
                // Use the first vertex normal for the entire face (simplification)
                self.normals[face.normal_indices[0]]
            } else {
                // Compute face normal if no normals are provided
                (v1 - v0).cross(v2 - v0).normalize()
            };

            // Create triangle
            triangles.push(Triangle {
                v0,
                v1,
                v2,
                normal,
            });
        }

        triangles
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