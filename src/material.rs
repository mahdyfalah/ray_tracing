#[derive(Debug, Clone)]
pub struct Material {
    pub color: [f64; 3], // RGB color
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    /// Creates a new material
    pub fn new(color: [f64; 3], ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}