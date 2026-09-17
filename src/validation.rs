use crate::bone::{Bone, BoneHierarchy};
use crate::error::AvatarResult;
use std::fs;
use std::path::Path;

pub fn export_bone_to_json<P: AsRef<Path>>(bone: &Bone, path: P) -> AvatarResult<()> {
    let json = serde_json::to_string_pretty(bone)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn import_bone_from_json<P: AsRef<Path>>(path: P) -> AvatarResult<Bone> {
    let content = fs::read_to_string(path)?;
    let bone: Bone = serde_json::from_str(&content)?;
    Ok(bone)
}

pub fn export_bones_individually<P: AsRef<Path>>(
    hierarchy: &BoneHierarchy,
    dir: P,
) -> AvatarResult<()> {
    let dir = dir.as_ref();
    fs::create_dir_all(dir)?;

    for bone in hierarchy.bones.values() {
        let file = dir.join(format!("{}.json", bone.id));
        export_bone_to_json(bone, file)?;
    }

    let meta = serde_json::to_string_pretty(hierarchy)?;
    fs::write(dir.join("_hierarchy.json"), meta)?;
    Ok(())
}

pub fn import_bones_individually<P: AsRef<Path>>(path: P) -> AvatarResult<BoneHierarchy> {
    let path = path.as_ref();
    let meta_path = path.join("_hierarchy.json");
    let hierarchy: BoneHierarchy = if meta_path.exists() {
        let content = fs::read_to_string(meta_path)?;
        serde_json::from_str(&content)?
    } else {
        BoneHierarchy::new("Imported Hierarchy".to_string())
    };

    let mut imported = hierarchy;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() && entry.file_name().to_string_lossy().ends_with(".json") {
            let name = entry.file_name();
            if name == "_hierarchy.json" {
                continue;
            }
            let bone: Bone = import_bone_from_json(entry.path())?;
            imported.bones.insert(bone.id.clone(), bone);
        }
    }

    Ok(imported)
}

pub fn export_bone_to_binary<P: AsRef<Path>>(bone: &Bone, path: P) -> AvatarResult<()> {
    let bytes = serde_json::to_vec(bone)?;
    fs::write(path, bytes)?;
    Ok(())
}

pub fn import_bone_from_binary<P: AsRef<Path>>(path: P) -> AvatarResult<Bone> {
    let bytes = fs::read(path)?;
    let bone: Bone = serde_json::from_slice(&bytes)?;
    Ok(bone)
}
