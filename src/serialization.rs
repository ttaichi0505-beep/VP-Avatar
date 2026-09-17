use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoneType {
    Armature,
    Head,
    Spine,
    Torso,
    Arm,
    Hand,
    Finger,
    Leg,
    Foot,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BoneTransform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bone {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub bone_type: BoneType,
    pub transform: BoneTransform,
    pub custom_properties: HashMap<String, String>,
}

impl Bone {
    pub fn new(name: String, parent_id: Option<String>, bone_type: BoneType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            parent_id,
            bone_type,
            transform: BoneTransform::default(),
            custom_properties: HashMap::new(),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.transform.position = [x, y, z];
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.transform.rotation = [x, y, z, w];
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.transform.scale = [x, y, z];
    }

    pub fn set_property(&mut self, key: String, value: String) {
        self.custom_properties.insert(key, value);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BoneHierarchy {
    pub name: String,
    pub bones: HashMap<String, Bone>,
    pub root_ids: Vec<String>,
}

impl BoneHierarchy {
    pub fn new(name: String) -> Self {
        Self {
            name,
            bones: HashMap::new(),
            root_ids: Vec::new(),
        }
    }

    pub fn add_bone(&mut self, bone: Bone) {
        if bone.parent_id.is_none() && !self.root_ids.iter().any(|id| *id == bone.id) {
            self.root_ids.push(bone.id.clone());
        }
        self.bones.insert(bone.id.clone(), bone);
    }

    pub fn get_bone(&self, id: &str) -> Option<&Bone> {
        self.bones.get(id)
    }

    pub fn get_children(&self, bone_id: &str) -> Vec<String> {
        self.bones
            .values()
            .filter(|bone| bone.parent_id.as_deref() == Some(bone_id))
            .map(|bone| bone.id.clone())
            .collect()
    }

    pub fn get_bones_by_type(&self, bone_type: BoneType) -> Vec<&Bone> {
        self.bones
            .values()
            .filter(|bone| bone.bone_type == bone_type)
            .collect()
    }

    pub fn bone_count(&self) -> usize {
        self.bones.len()
    }
}

impl Default for BoneTransform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}
