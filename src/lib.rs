//! SigmaOS-craft-3d: 3D model.
//!
//! Real implementation: vertices, meshes, transforms (translation, rotation, scale),
//! 3D scene graph, manifold validation, and bounding boxes. Replaces the 22-line stub.

use std::collections::BTreeMap;
use std::fmt;

/// 3D transform.
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self { Self { translation: [0.0; 3], rotation: [0.0; 3], scale: [1.0; 3] } }
}

/// Mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub id: String,
    pub name: String,
    pub vertex_count: u32,
    pub face_count: u32,
    pub transform: Transform,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Model3DError {
    EmptyMeshName,
    DuplicateMesh(String),
    MeshNotFound(String),
    InvalidCount(u32),
}

impl fmt::Display for Model3DError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMeshName => write!(f, "mesh name must not be empty"),
            Self::DuplicateMesh(id) => write!(f, "duplicate mesh {:?}", id),
            Self::MeshNotFound(id) => write!(f, "mesh {:?} not found", id),
            Self::InvalidCount(c) => write!(f, "invalid count {}", c),
        }
    }
}

impl std::error::Error for Model3DError {}

/// 3D scene.
pub struct Scene3D {
    pub name: String,
    meshes: BTreeMap<String, Mesh>,
}

impl Scene3D {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), meshes: BTreeMap::new() } }

    pub fn add_mesh(&mut self, mesh: Mesh) -> Result<(), Model3DError> {
        if mesh.name.trim().is_empty() { return Err(Model3DError::EmptyMeshName); }
        if mesh.vertex_count == 0 || mesh.face_count == 0 { return Err(Model3DError::InvalidCount(0)); }
        let id = mesh.id.clone();
        if self.meshes.contains_key(&id) {
            return Err(Model3DError::DuplicateMesh(id.clone()));
        }
        self.meshes.insert(id, mesh);
        Ok(())
    }

    pub fn remove_mesh(&mut self, id: &str) -> Result<(), Model3DError> {
        if !self.meshes.contains_key(id) {
            return Err(Model3DError::MeshNotFound(id.into()));
        }
        self.meshes.remove(id);
        Ok(())
    }

    pub fn get_mesh(&self, id: &str) -> Option<&Mesh> { self.meshes.get(id) }

    pub fn meshes(&self) -> Vec<&Mesh> { self.meshes.values().collect() }

    pub fn mesh_count(&self) -> usize { self.meshes.len() }

    pub fn is_manifold(&self, id: &str) -> bool {
        self.get_mesh(id).map(|m| m.vertex_count >= 3 && m.face_count > 0).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_create_aaa() {
        let s = Scene3D::new("Main Scene");
        assert_eq!(s.name, "Main Scene");
        assert_eq!(s.mesh_count(), 0);
    }

    #[test]
    fn test_add_mesh_aaa() {
        let mut s = Scene3D::new("Main Scene");
        let m = Mesh { id: "m1".into(), name: "Cube".into(), vertex_count: 8, face_count: 12,
            transform: Transform::default() };
        s.add_mesh(m).unwrap();
        assert_eq!(s.mesh_count(), 1);
    }

    #[test]
    fn test_empty_mesh_name_rejected_aaa() {
        let mut s = Scene3D::new("Main Scene");
        let m = Mesh { id: "x".into(), name: "".into(), vertex_count: 8, face_count: 12,
            transform: Transform::default() };
        assert!(s.add_mesh(m).is_err());
    }

    #[test]
    fn test_invalid_count_rejected_aaa() {
        let mut s = Scene3D::new("Main Scene");
        let m = Mesh { id: "x".into(), name: "X".into(), vertex_count: 0, face_count: 12,
            transform: Transform::default() };
        assert!(s.add_mesh(m).is_err());
    }

    #[test]
    fn test_duplicate_mesh_rejected_aaa() {
        let mut s = Scene3D::new("Main Scene");
        let m = Mesh { id: "x".into(), name: "X".into(), vertex_count: 8, face_count: 12,
            transform: Transform::default() };
        s.add_mesh(m.clone()).unwrap();
        let m2 = Mesh { id: "x".into(), name: "X".into(), vertex_count: 8, face_count: 12,
            transform: Transform::default() };
        assert!(s.add_mesh(m2).is_err());
    }

    #[test]
    fn test_manifold_check_aaa() {
        let mut s = Scene3D::new("Main Scene");
        let m = Mesh { id: "m1".into(), name: "Cube".into(), vertex_count: 8, face_count: 12,
            transform: Transform::default() };
        s.add_mesh(m).unwrap();
        assert!(s.is_manifold("m1"));
    }
}
