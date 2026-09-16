//! Material and texture support for VRM avatars

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a material with PBR properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    /// Unique identifier
    pub id: String,

    /// Material name
    pub name: String,

    /// Base color [R, G, B, A]
    pub base_color: [f32; 4],

    /// Metallic factor (0.0 - 1.0)
    pub metallic: f32,

    /// Roughness factor (0.0 - 1.0)
    pub roughness: f32,

    /// Normal map texture path
    pub normal_texture: Option<String>,

    /// Base color texture path
    pub base_color_texture: Option<String>,

    /// Metallic roughness texture path
    pub metallic_roughness_texture: Option<String>,

    /// Emissive color [R, G, B]
    pub emissive: [f32; 3],

    /// Emissive texture path
    pub emissive_texture: Option<String>,

    /// Alpha mode (Opaque, Mask, Blend)
    pub alpha_mode: AlphaMode,

    /// Alpha cutoff (for Mask mode)
    pub alpha_cutoff: f32,

    /// Additional properties
    pub properties: HashMap<String, String>,
}

/// Alpha blending mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlphaMode {
    /// Fully opaque
    Opaque,
    /// Binary transparency
    Mask,
    /// Smooth transparency
    Blend,
}

impl Material {
    /// Creates a new material with default values
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            base_color: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            normal_texture: None,
            base_color_texture: None,
            metallic_roughness_texture: None,
            emissive: [0.0, 0.0, 0.0],
            emissive_texture: None,
            alpha_mode: AlphaMode::Opaque,
            alpha_cutoff: 0.5,
            properties: HashMap::new(),
        }
    }

    /// Sets base color
    pub fn set_base_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.base_color = [r, g, b, a];
    }

    /// Sets PBR properties
    pub fn set_pbr(&mut self, metallic: f32, roughness: f32) {
        self.metallic = metallic.clamp(0.0, 1.0);
        self.roughness = roughness.clamp(0.0, 1.0);
    }

    /// Sets emissive color
    pub fn set_emissive(&mut self, r: f32, g: f32, b: f32) {
        self.emissive = [r, g, b];
    }

    /// Sets texture path
    pub fn set_base_color_texture(&mut self, path: String) {
        self.base_color_texture = Some(path);
    }

    /// Sets normal map
    pub fn set_normal_texture(&mut self, path: String) {
        self.normal_texture = Some(path);
    }

    /// Sets property
    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new("Default".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let material = Material::new("Skin".to_string());
        assert_eq!(material.name, "Skin");
        assert_eq!(material.base_color, [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_material_pbr() {
        let mut material = Material::new("Metal".to_string());
        material.set_pbr(1.0, 0.2);

        assert_eq!(material.metallic, 1.0);
        assert_eq!(material.roughness, 0.2);
    }
}
