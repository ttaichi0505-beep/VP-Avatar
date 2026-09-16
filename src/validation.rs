//! Validation module for avatar skeleton and mesh integrity

use crate::bone::{Bone, BoneHierarchy};
use crate::error::AvatarResult;

/// Validation result with details
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether validation passed
    pub is_valid: bool,

    /// List of errors found
    pub errors: Vec<String>,

    /// List of warnings
    pub warnings: Vec<String>,

    /// Statistics
    pub stats: ValidationStats,
}

/// Validation statistics
#[derive(Debug, Clone)]
pub struct ValidationStats {
    /// Total bones
    pub bone_count: usize,

    /// Root bones
    pub root_count: usize,

    /// Bones with no children
    pub leaf_count: usize,

    /// Bones with multiple children
    pub branch_count: usize,
}

/// Validates a bone hierarchy
pub fn validate_hierarchy(hierarchy: &BoneHierarchy) -> ValidationReport {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check for orphaned bones
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

    // Check for circular references
    for (bone_id, _) in &hierarchy.bones {
        if has_circular_reference(hierarchy, bone_id) {
            errors.push(format!("Circular reference detected in bone '{}'", bone_id));
        }
    }

    // Verify root bones
    if hierarchy.root_ids.is_empty() {
        warnings.push("Hierarchy has no root bones".to_string());
    }

    // Calculate statistics
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

    let is_valid = errors.is_empty();

    ValidationReport {
        is_valid,
        errors,
        warnings,
        stats,
    }
}

/// Checks if there's a circular reference in the hierarchy
fn has_circular_reference(hierarchy: &BoneHierarchy, start_id: &str) -> bool {
    let mut visited = std::collections::HashSet::new();
    let mut stack = vec![start_id.to_string()];

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            return true; // Circular reference found
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
}
