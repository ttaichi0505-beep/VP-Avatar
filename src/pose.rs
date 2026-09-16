//! Keyframe and static pose data for avatars

use serde::{Deserialize, Serialize};

/// Represents a static pose or keyframe data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pose {
    /// Pose name/identifier
    pub name: String,

    /// Pose data indexed by bone ID
    pub bone_poses: std::collections::HashMap<String, BonePose>,

    /// Metadata
    pub description: Option<String>,

    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Transform data for a single bone at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonePose {
    /// Bone ID
    pub bone_id: String,

    /// Position [x, y, z]
    pub position: [f32; 3],

    /// Rotation as quaternion [x, y, z, w]
    pub rotation: [f32; 4],

    /// Scale [x, y, z]
    pub scale: [f32; 3],
}

impl Pose {
    /// Creates a new pose
    pub fn new(name: String) -> Self {
        Self {
            name,
            bone_poses: std::collections::HashMap::new(),
            description: None,
            tags: Vec::new(),
        }
    }

    /// Adds a bone pose
    pub fn add_bone_pose(&mut self, bone_id: String, pose: BonePose) {
        self.bone_poses.insert(bone_id, pose);
    }

    /// Gets a bone pose
    pub fn get_bone_pose(&self, bone_id: &str) -> Option<&BonePose> {
        self.bone_poses.get(bone_id)
    }

    /// Sets description
    pub fn set_description(&mut self, desc: String) {
        self.description = Some(desc);
    }

    /// Adds a tag
    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    /// Gets bone count
    pub fn bone_count(&self) -> usize {
        self.bone_poses.len()
    }
}

impl BonePose {
    /// Creates a new bone pose
    pub fn new(bone_id: String) -> Self {
        Self {
            bone_id,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity
            scale: [1.0, 1.0, 1.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pose_creation() {
        let pose = Pose::new("Idle".to_string());
        assert_eq!(pose.name, "Idle");
        assert_eq!(pose.bone_count(), 0);
    }

    #[test]
    fn test_bone_pose() {
        let mut pose = Pose::new("Rest".to_string());
        let mut bone_pose = BonePose::new("Head".to_string());
        bone_pose.position = [0.0, 1.5, 0.0];

        pose.add_bone_pose("Head".to_string(), bone_pose);
        assert_eq!(pose.bone_count(), 1);
    }
}
