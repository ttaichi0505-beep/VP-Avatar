//! Avatar struct and utilities for handling VRM files

use crate::error::AvatarResult;
use serde::{Deserialize, Serialize};
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
            },
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
