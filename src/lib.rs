#[derive(Debug, Clone)]
pub struct Mesh { pub name: String, pub vertex_count: usize, pub face_count: usize }
impl Mesh {
    pub fn new(name: impl Into<String>, vertices: usize, faces: usize) -> Self {
        Self { name: name.into(), vertex_count: vertices, face_count: faces }
    }
    pub fn is_manifold(&self) -> bool { self.face_count > 0 && self.vertex_count >= 3 }
}

#[derive(Debug, Default)]
pub struct Scene { pub name: String, meshes: Vec<Mesh> }
impl Scene {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), meshes: Vec::new() } }
    pub fn add_mesh(&mut self, m: Mesh) { self.meshes.push(m); }
    pub fn mesh_count(&self) -> usize { self.meshes.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_mesh() { assert!(Mesh::new("cube", 8, 12).is_manifold()); }
}
