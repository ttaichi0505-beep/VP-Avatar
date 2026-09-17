//! Validation module for avatar skeleton and mesh integrity

use crate::bone::{Bone, BoneHierarchy};
use crate::error::{AvatarError, AvatarResult};

/// Validation result with details
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub stats: ValidationStats,
}

#[derive(Debug, Clone)]
pub struct ValidationStats {
    pub bone_count: usize,
    pub root_count: usize,
    pub leaf_count: usize,
    pub branch_count: usize,
}

pub fn validate_gltf_2_0_bytes(bytes: &[u8]) -> AvatarResult<()> {
    if bytes.is_empty() {
        return Err(AvatarError::InvalidFormat("input is empty".to_string()));
    }

    if bytes.starts_with(b"glTF") {
        validate_glb_bytes(bytes)
    } else {
        validate_json_gltf_bytes(bytes)
    }
}

fn validate_glb_bytes(bytes: &[u8]) -> AvatarResult<()> {
    if bytes.len() < 12 {
        return Err(AvatarError::InvalidFormat(
            "GLB input is shorter than the required 12-byte header".to_string(),
        ));
    }

    if &bytes[0..4] != b"glTF" {
        return Err(AvatarError::InvalidFormat(
            "invalid GLB magic, expected 'glTF'".to_string(),
        ));
    }

    let version = u32::from_le_bytes(
        bytes[4..8]
            .try_into()
            .map_err(|_| AvatarError::InvalidFormat("GLB header is truncated".to_string()))?,
    );
    if version != 2 {
        return Err(AvatarError::InvalidFormat(format!(
            "unsupported GLB version: {version}; expected 2"
        )));
    }

    let total_length = u32::from_le_bytes(
        bytes[8..12]
            .try_into()
            .map_err(|_| AvatarError::InvalidFormat("GLB totalLength is truncated".to_string()))?,
    ) as usize;
    if total_length != bytes.len() {
        return Err(AvatarError::InvalidFormat(format!(
            "GLB totalLength mismatch: file size is {}, header says {}",
            bytes.len(),
            total_length
        )));
    }

    let mut offset = 12usize;
    let mut saw_json = false;
    let mut json_chunk_start = None;
    let mut json_chunk_end = None;

    while offset + 8 <= bytes.len() {
        let chunk_length = u32::from_le_bytes(
            bytes[offset..offset + 4]
                .try_into()
                .map_err(|_| AvatarError::InvalidFormat("chunk length is truncated".to_string()))?,
        ) as usize;
        let chunk_type = &bytes[offset + 4..offset + 8];

        let chunk_data_start = offset
            .checked_add(8)
            .ok_or_else(|| AvatarError::InvalidFormat("chunk offset overflow".to_string()))?;
        let chunk_data_end = chunk_data_start
            .checked_add(chunk_length)
            .ok_or_else(|| AvatarError::InvalidFormat("chunk length overflow".to_string()))?;

        if chunk_data_end > bytes.len() {
            return Err(AvatarError::InvalidFormat(
                "GLB chunk extends beyond file size".to_string(),
            ));
        }

        if chunk_type == b"JSON" {
            saw_json = true;
            json_chunk_start = Some(chunk_data_start);
            json_chunk_end = Some(chunk_data_end);
        }

        offset = chunk_data_end;
        if offset == bytes.len() {
            break;
        }
    }

    if !saw_json {
        return Err(AvatarError::InvalidFormat(
            "GLB does not contain a JSON chunk".to_string(),
        ));
    }

    let json_start = json_chunk_start.unwrap_or(0);
    let json_end = json_chunk_end.unwrap_or(bytes.len());
    let json_bytes = &bytes[json_start..json_end];
    validate_json_document(json_bytes)
}

fn validate_json_gltf_bytes(bytes: &[u8]) -> AvatarResult<()> {
    validate_json_document(bytes)
}

fn validate_json_document(bytes: &[u8]) -> AvatarResult<()> {
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|err| AvatarError::Parse(format!("JSON parse failed: {err}")))?;

    let asset = value
        .get("asset")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            AvatarError::InvalidFormat("JSON lacks an 'asset' object".to_string())
        })?;

    let version = asset
        .get("version")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            AvatarError::InvalidFormat("JSON asset.version is missing or not a string".to_string())
        })?;

    if version != "2.0" {
        return Err(AvatarError::InvalidFormat(format!(
            "unsupported glTF asset.version: {version}; expected 2.0"
        )));
    }

    Ok(())
}

pub fn validate_hierarchy(hierarchy: &BoneHierarchy) -> ValidationReport {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for (bone_id, bone) in &hierarchy.bones {
        if let Some(parent_id) = &bone.parent_id {
            if !hierarchy.bones.contains_key(parent_id) {
                errors.push(format!(
                    "Bone '{}' references non-existent parent '{}'",
                    bone_id, parent_id
                ));
            }
        }
    }

    for (bone_id, _) in &hierarchy.bones {
        if has_circular_reference(hierarchy, bone_id) {
            errors.push(format!("Circular reference detected in bone '{}'", bone_id));
        }
    }

    if hierarchy.root_ids.is_empty() {
        warnings.push("Hierarchy has no root bones".to_string());
    }

    let root_count = hierarchy.root_ids.len();
    let bone_count = hierarchy.bones.len();
    let leaf_count = hierarchy
        .bones
        .values()
        .filter(|bone| hierarchy.get_children(&bone.id).is_empty())
        .count();
    let branch_count = hierarchy
        .bones
        .values()
        .filter(|bone| hierarchy.get_children(&bone.id).len() > 1)
        .count();

    let stats = ValidationStats {
        bone_count,
        root_count,
        leaf_count,
        branch_count,
    };

    ValidationReport {
        is_valid: errors.is_empty(),
        errors,
        warnings,
        stats,
    }
}

fn has_circular_reference(hierarchy: &BoneHierarchy, start_id: &str) -> bool {
    let mut visited = std::collections::HashSet::new();
    let mut stack = vec![start_id.to_string()];

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            return true;
        }
        visited.insert(current.clone());

        if let Some(bone) = hierarchy.get_bone(&current) {
            if let Some(parent_id) = &bone.parent_id {
                stack.push(parent_id.clone());
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bone::{Bone, BoneType};

    #[test]
    fn test_valid_hierarchy() {
        let mut hierarchy = BoneHierarchy::new("Test".to_string());
        let bone = Bone::new("Root".to_string(), None, BoneType::Armature);
        hierarchy.add_bone(bone);

        let report = validate_hierarchy(&hierarchy);
        assert!(report.is_valid);
        assert_eq!(report.stats.bone_count, 1);
    }

    #[test]
    fn test_validate_json_gltf_bytes() {
        let json = br#"{"asset":{"version":"2.0"}}"#;
        assert!(validate_json_gltf_bytes(json).is_ok());
    }

    #[test]
    fn test_validate_invalid_asset_version() {
        let json = br#"{"asset":{"version":"1.0"}}"#;
        assert!(validate_json_gltf_bytes(json).is_err());
    }

    #[test]
    fn test_validate_glb_accepts_valid_header() {
        let mut bytes = vec![0u8; 12 + 8 + 8];
        bytes[0..4].copy_from_slice(b"glTF");
        bytes[4..8].copy_from_slice(&(2u32).to_le_bytes());
        bytes[8..12].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
        bytes[12..16].copy_from_slice(&(8u32).to_le_bytes());
        bytes[16..20].copy_from_slice(b"JSON");
        bytes[20..].fill(b' ');

        let payload = br#"{"asset":{"version":"2.0"}}"#;
        let payload_len = payload.len() as u32;
        bytes[12..16].copy_from_slice(&payload_len.to_le_bytes());
        bytes[20..20 + payload.len()].copy_from_slice(payload);

        assert!(validate_glb_bytes(&bytes).is_ok());
    }
}
