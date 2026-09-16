//! Export module for various file formats

use crate::avatar::Avatar;
use crate::bone::BoneHierarchy;
use crate::error::AvatarResult;
use serde_json;
use std::fs;
use std::path::Path;

/// Exports avatar to JSON format
pub fn export_avatar_to_json<P: AsRef<Path>>(avatar: &Avatar, path: P) -> AvatarResult<()> {
    let json = serde_json::to_string_pretty(avatar)?;
    fs::write(path, json)?;
    Ok(())
}

/// Imports avatar from JSON format
pub fn import_avatar_from_json<P: AsRef<Path>>(path: P) -> AvatarResult<Avatar> {
    let content = fs::read_to_string(path)?;
    let avatar = serde_json::from_str(&content)?;
    Ok(avatar)
}

/// Exports avatar to JSON Lines format (one object per line)
pub fn export_avatar_to_jsonl<P: AsRef<Path>>(avatar: &Avatar, path: P) -> AvatarResult<()> {
    let mut content = String::new();

    // Metadata line
    content.push_str(&serde_json::to_string(&avatar.metadata)?);
    content.push('\n');

    // Meshes
    for (_, mesh) in &avatar.meshes {
        content.push_str(&serde_json::to_string(mesh)?);
        content.push('\n');
    }

    // Materials
    for (_, material) in &avatar.materials {
        content.push_str(&serde_json::to_string(material)?);
        content.push('\n');
    }

    // Bones
    if let Some(skeleton) = &avatar.skeleton {
        for (_, bone) in &skeleton.bones {
            content.push_str(&serde_json::to_string(bone)?);
            content.push('\n');
        }
    }

    // Poses
    for (_, pose) in &avatar.poses {
        content.push_str(&serde_json::to_string(pose)?);
        content.push('\n');
    }

    fs::write(path, content)?;
    Ok(())
}

/// Exports avatar structure as human-readable summary
pub fn export_avatar_summary<P: AsRef<Path>>(avatar: &Avatar, path: P) -> AvatarResult<()> {
    let stats = avatar.get_statistics();
    let skeleton_info = avatar
        .get_skeleton()
        .map(|s| format!("Skeleton: {} bones", s.bone_count()))
        .unwrap_or_else(|| "No skeleton".to_string());

    let summary = format!(
        "Avatar: {}\n\
         Author: {}\n\
         Version: {}\n\
         \n\
         Statistics:\n\
         - Meshes: {}\n\
         - Materials: {}\n\
         - Bones: {}\n\
         - Poses: {}\n\
         - Total Vertices: {}\n\
         - Total Faces: {}\n\
         \n\
         {}",
        avatar.name,
        avatar
            .metadata
            .author
            .as_deref()
            .unwrap_or("Unknown"),
        avatar
            .metadata
            .version
            .as_deref()
            .unwrap_or("1.0"),
        stats.mesh_count,
        stats.material_count,
        stats.bone_count,
        stats.pose_count,
        stats.total_vertices,
        stats.total_faces,
        skeleton_info
    );

    fs::write(path, summary)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_avatar_json_export() {
        let avatar = Avatar::new("Test".to_string());
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        export_avatar_to_json(&avatar, &file_path).unwrap();
        assert!(file_path.exists());
    }

    #[test]
    fn test_avatar_summary_export() {
        let avatar = Avatar::new("Test".to_string());
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("summary.txt");

        export_avatar_summary(&avatar, &file_path).unwrap();
        assert!(file_path.exists());
    }
}
