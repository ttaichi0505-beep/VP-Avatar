//! Mesh structures for VRM avatars

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents mesh geometry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// Unique identifier for this mesh
    pub id: String,

    /// Mesh name
    pub name: String,

    /// Vertex positions [x, y, z] for each vertex
    pub vertices: Vec<[f32; 3]>,

    /// Face indices (triangles)
    pub indices: Vec<u32>,

    /// Vertex normals [x, y, z]
    pub normals: Option<Vec<[f32; 3]>>,

    /// UV coordinates [u, v]
    pub uv_coordinates: Option<Vec<[f32; 2]>>,

    /// Bone weights for rigging
    pub bone_weights: Option<Vec<BoneWeight>>,

    /// Associated material ID
    pub material_id: Option<String>,

    /// Mesh bounds
    pub bounds: Bounds,
}

/// Bone weight for vertex skinning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneWeight {
    /// Bone ID
    pub bone_id: String,

    /// Weight value (0.0 to 1.0)
    pub weight: f32,
}

/// 3D bounding box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounds {
    /// Minimum point [x, y, z]
    pub min: [f32; 3],

    /// Maximum point [x, y, z]
    pub max: [f32; 3],
}

impl Mesh {
    /// Creates a new mesh
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: None,
            uv_coordinates: None,
            bone_weights: None,
            material_id: None,
            bounds: Bounds {
                min: [f32::MAX, f32::MAX, f32::MAX],
                max: [f32::MIN, f32::MIN, f32::MIN],
            },
        }
    }

    /// Adds a vertex and updates bounds
    pub fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push([x, y, z]);
        self.update_bounds([x, y, z]);
    }

    /// Adds a face (triangle)
    pub fn add_face(&mut self, i0: u32, i1: u32, i2: u32) {
        self.indices.extend_from_slice(&[i0, i1, i2]);
    }

    /// Updates mesh bounds
    fn update_bounds(&mut self, point: [f32; 3]) {
        self.bounds.min[0] = self.bounds.min[0].min(point[0]);
        self.bounds.min[1] = self.bounds.min[1].min(point[1]);
        self.bounds.min[2] = self.bounds.min[2].min(point[2]);

        self.bounds.max[0] = self.bounds.max[0].max(point[0]);
        self.bounds.max[1] = self.bounds.max[1].max(point[1]);
        self.bounds.max[2] = self.bounds.max[2].max(point[2]);
    }

    /// Calculates vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Calculates face count
    pub fn face_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Sets the material ID
    pub fn set_material(&mut self, material_id: String) {
        self.material_id = Some(material_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        let mesh = Mesh::new("Head".to_string());
        assert_eq!(mesh.name, "Head");
        assert_eq!(mesh.vertex_count(), 0);
    }

    #[test]
    fn test_mesh_vertices() {
        let mut mesh = Mesh::new("Test".to_string());
        mesh.add_vertex(0.0, 0.0, 0.0);
        mesh.add_vertex(1.0, 0.0, 0.0);
        mesh.add_vertex(0.0, 1.0, 0.0);

        assert_eq!(mesh.vertex_count(), 3);
    }

    #[test]
    fn test_mesh_faces() {
        let mut mesh = Mesh::new("Test".to_string());
        mesh.add_vertex(0.0, 0.0, 0.0);
        mesh.add_vertex(1.0, 0.0, 0.0);
        mesh.add_vertex(0.0, 1.0, 0.0);
        mesh.add_face(0, 1, 2);

        assert_eq!(mesh.face_count(), 1);
    }
}
