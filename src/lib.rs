// SPDX-License-Identifier: MIT
//! SigmaOS-craft-3d — Real 3D Modeling, Mesh & Vector Math Core
//!
//! Principles:
//! - NO STUBS, NO MOCKS: Mathematical precision and real 3D geometry engine
//! - ARRANGE-ACT-ASSERT: All functionality rigorously validated with AAA tests
//! - PERFORMANCE & CACHE-CONSCIOUS: Compact representations for high throughput

#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::ops::{Add, Mul, Sub};

/// 3-Dimensional floating point vector for spatial coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Create a new 3D vector.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Calculate Euclidean magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Return normalized unit vector (direction).
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::new(0.0, 0.0, 0.0)
        } else {
            Self::new(self.x / mag, self.y / mag, self.z / mag)
        }
    }

    /// Compute dot product with another vector.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Compute cross product with another vector.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// 3D Triangle face with 3 vertices in counter-clockwise winding order.
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl Triangle {
    /// Create a new triangle from 3 vertices.
    pub const fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self { v0, v1, v2 }
    }

    /// Compute surface normal vector.
    pub fn normal(&self) -> Vec3 {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        edge1.cross(&edge2).normalize()
    }

    /// Compute planar surface area using cross-product magnitude.
    pub fn area(&self) -> f64 {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        0.5 * edge1.cross(&edge2).magnitude()
    }
}

/// Polygonal 3D Mesh representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub name: String,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    /// Create a new empty named mesh.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            triangles: Vec::new(),
        }
    }

    /// Add a triangle face to the mesh.
    pub fn add_triangle(&mut self, t: Triangle) {
        self.triangles.push(t);
    }

    /// Total number of triangular faces.
    pub fn face_count(&self) -> usize {
        self.triangles.len()
    }

    /// Sum of surface areas across all constituent triangles.
    pub fn surface_area(&self) -> f64 {
        self.triangles.iter().map(|t| t.area()).sum()
    }
}

/// Perspective 3D Camera with position, look target, and field of view.
#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov_degrees: f64,
}

impl Camera {
    /// Create a new camera.
    pub const fn new(position: Vec3, target: Vec3, fov_degrees: f64) -> Self {
        Self {
            position,
            target,
            fov_degrees,
        }
    }

    /// Compute forward look ray unit direction.
    pub fn ray_direction(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_magnitude() {
        // Arrange
        let v = Vec3::new(3.0, 4.0, 0.0);
        // Act
        let mag = v.magnitude();
        // Assert
        assert!((mag - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_vec3_dot() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -5.0, 6.0);
        // Act
        let d = a.dot(&b);
        // Assert: 1*4 + 2*(-5) + 3*6 = 4 - 10 + 18 = 12
        assert!((d - 12.0).abs() < 1e-6);
    }

    #[test]
    fn test_vec3_cross() {
        // Arrange
        let x_axis = Vec3::new(1.0, 0.0, 0.0);
        let y_axis = Vec3::new(0.0, 1.0, 0.0);
        // Act
        let z_axis = x_axis.cross(&y_axis);
        // Assert
        assert_eq!(z_axis, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_triangle_normal() {
        // Arrange
        let t = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        // Act
        let n = t.normal();
        // Assert: CCW in XY plane points along +Z
        assert_eq!(n, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_mesh_face_count() {
        // Arrange
        let mut mesh = Mesh::new("CubeFace");
        let t1 = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        // Act
        mesh.add_triangle(t1);
        // Assert
        assert_eq!(mesh.face_count(), 1);
    }

    #[test]
    fn test_mesh_surface_area() {
        // Arrange: Right triangle with legs 2.0 and 3.0 -> area = 3.0
        let mut mesh = Mesh::new("Plane");
        mesh.add_triangle(Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
        ));
        // Act
        let area = mesh.surface_area();
        // Assert
        assert!((area - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_camera_ray_direction() {
        // Arrange
        let cam = Camera::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, 0.0), 60.0);
        // Act
        let dir = cam.ray_direction();
        // Assert: looking from +10Z to 0Z -> (0, 0, -1)
        assert_eq!(dir, Vec3::new(0.0, 0.0, -1.0));
    }
}
