//! Avatar struct and utilities for handling VRM files

use crate::bone::BoneHierarchy;
use crate::error::AvatarResult;
use crate::material::Material;
use crate::mesh::Mesh;
use crate::pose::Pose;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Represents a VRM avatar loaded from VRoid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Avatar {
    /// Name of the avatar
    pub name: String,

    /// Version of the VRM format
    pub vrm_version: String,

    /// Avatar metadata
    pub metadata: AvatarMetadata,

    /// Bone skeleton structure
    pub skeleton: Option<BoneHierarchy>,

    /// Meshes in the avatar
    pub meshes: HashMap<String, Mesh>,

    /// Materials used by meshes
    pub materials: HashMap<String, Material>,

    /// Available poses
    pub poses: HashMap<String, Pose>,
}

/// Metadata associated with a VRM avatar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarMetadata {
    /// Author/creator of the avatar
    pub author: Option<String>,

    /// Version of the avatar
    pub version: Option<String>,

    /// License information
    pub license: Option<String>,

    /// Additional information
    pub info: Option<String>,

    /// Custom properties
    pub custom_properties: HashMap<String, String>,
}

impl Avatar {
    /// Creates a new Avatar instance
    pub fn new(name: String) -> Self {
        Self {
            name,
            vrm_version: "0.0".to_string(),
            metadata: AvatarMetadata {
                author: None,
                version: None,
                license: None,
                info: None,
                custom_properties: HashMap::new(),
            },
            skeleton: None,
            meshes: HashMap::new(),
            materials: HashMap::new(),
            poses: HashMap::new(),
        }
    }

    /// Loads a VRM avatar from a file path
    pub fn load<P: AsRef<Path>>(path: P) -> AvatarResult<Self> {
        let _path = path.as_ref();
        // TODO: Implement VRM file loading
        Err(crate::error::AvatarError::LoadError(
            "VRM loading not yet implemented".to_string(),
        ))
    }

    /// Sets the author of the avatar
    pub fn set_author(&mut self, author: String) {
        self.metadata.author = Some(author);
    }

    /// Sets the version of the avatar
    pub fn set_version(&mut self, version: String) {
        self.metadata.version = Some(version);
    }

    /// Sets the skeleton/bone hierarchy
    pub fn set_skeleton(&mut self, skeleton: BoneHierarchy) {
        self.skeleton = Some(skeleton);
    }

    /// Gets a reference to the skeleton
    pub fn get_skeleton(&self) -> Option<&BoneHierarchy> {
        self.skeleton.as_ref()
    }

    /// Gets a mutable reference to the skeleton
    pub fn get_skeleton_mut(&mut self) -> Option<&mut BoneHierarchy> {
        self.skeleton.as_mut()
    }

    /// Adds a mesh to the avatar
    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.insert(mesh.id.clone(), mesh);
    }

    /// Gets a mesh by ID
    pub fn get_mesh(&self, mesh_id: &str) -> Option<&Mesh> {
        self.meshes.get(mesh_id)
    }

    /// Gets a mutable mesh by ID
    pub fn get_mesh_mut(&mut self, mesh_id: &str) -> Option<&mut Mesh> {
        self.meshes.get_mut(mesh_id)
    }

    /// Adds a material to the avatar
    pub fn add_material(&mut self, material: Material) {
        self.materials.insert(material.id.clone(), material);
    }

    /// Gets a material by ID
    pub fn get_material(&self, material_id: &str) -> Option<&Material> {
        self.materials.get(material_id)
    }

    /// Gets a mutable material by ID
    pub fn get_material_mut(&mut self, material_id: &str) -> Option<&mut Material> {
        self.materials.get_mut(material_id)
    }

    /// Adds a pose to the avatar
    pub fn add_pose(&mut self, pose: Pose) {
        self.poses.insert(pose.name.clone(), pose);
    }

    /// Gets a pose by name
    pub fn get_pose(&self, pose_name: &str) -> Option<&Pose> {
        self.poses.get(pose_name)
    }

    /// Gets a mutable pose by name
    pub fn get_pose_mut(&mut self, pose_name: &str) -> Option<&mut Pose> {
        self.poses.get_mut(pose_name)
    }

    /// Sets a custom property
    pub fn set_property(&mut self, key: String, value: String) {
        self.metadata.custom_properties.insert(key, value);
    }

    /// Gets summary statistics
    pub fn get_statistics(&self) -> AvatarStatistics {
        AvatarStatistics {
            mesh_count: self.meshes.len(),
            material_count: self.materials.len(),
            bone_count: self.skeleton.as_ref().map(|s| s.bone_count()).unwrap_or(0),
            pose_count: self.poses.len(),
            total_vertices: self.meshes.values().map(|m| m.vertex_count()).sum(),
            total_faces: self.meshes.values().map(|m| m.face_count()).sum(),
        }
    }
}

/// Avatar statistics
#[derive(Debug, Clone)]
pub struct AvatarStatistics {
    /// Number of meshes
    pub mesh_count: usize,

    /// Number of materials
    pub material_count: usize,

    /// Number of bones
    pub bone_count: usize,

    /// Number of poses
    pub pose_count: usize,

    /// Total vertices across all meshes
    pub total_vertices: usize,

    /// Total faces across all meshes
    pub total_faces: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bone::{Bone, BoneHierarchy, BoneType};
    use crate::material::Material;
    use crate::mesh::Mesh;
    use crate::pose::Pose;

    #[test]
    fn test_avatar_creation() {
        let avatar = Avatar::new("Test Avatar".to_string());
        assert_eq!(avatar.name, "Test Avatar");
    }

    #[test]
    fn test_avatar_metadata() {
        let mut avatar = Avatar::new("Test Avatar".to_string());
        avatar.set_author("Author Name".to_string());
        assert_eq!(avatar.metadata.author, Some("Author Name".to_string()));
    }

    #[test]
    fn test_avatar_skeleton() {
        let mut avatar = Avatar::new("Test Avatar".to_string());
        let skeleton = BoneHierarchy::new("Skeleton".to_string());
        avatar.set_skeleton(skeleton);

        assert!(avatar.get_skeleton().is_some());
    }

    #[test]
    fn test_avatar_meshes() {
        let mut avatar = Avatar::new("Test Avatar".to_string());
        let mesh = Mesh::new("Head".to_string());

        avatar.add_mesh(mesh.clone());
        assert_eq!(avatar.meshes.len(), 1);
        assert!(avatar.get_mesh(&mesh.id).is_some());
    }

    #[test]
    fn test_avatar_materials() {
        let mut avatar = Avatar::new("Test Avatar".to_string());
        let material = Material::new("Skin".to_string());

        avatar.add_material(material.clone());
        assert_eq!(avatar.materials.len(), 1);
        assert!(avatar.get_material(&material.id).is_some());
    }

    #[test]
    fn test_avatar_poses() {
        let mut avatar = Avatar::new("Test Avatar".to_string());
        let pose = Pose::new("Idle".to_string());

        avatar.add_pose(pose);
        assert_eq!(avatar.poses.len(), 1);
        assert!(avatar.get_pose("Idle").is_some());
    }

    #[test]
    fn test_avatar_statistics() {
        let mut avatar = Avatar::new("Test Avatar".to_string());
        let mut mesh = Mesh::new("Head".to_string());
        mesh.add_vertex(0.0, 0.0, 0.0);
        mesh.add_vertex(1.0, 0.0, 0.0);
        mesh.add_vertex(0.0, 1.0, 0.0);
        mesh.add_face(0, 1, 2);

        avatar.add_mesh(mesh);
        avatar.add_material(Material::new("Skin".to_string()));

        let stats = avatar.get_statistics();
        assert_eq!(stats.mesh_count, 1);
        assert_eq!(stats.material_count, 1);
        assert_eq!(stats.total_vertices, 3);
        assert_eq!(stats.total_faces, 1);
    }
}
