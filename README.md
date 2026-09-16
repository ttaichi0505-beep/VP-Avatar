# VP-Avatar

Rust-based VRM avatar handling library for VRoid avatars with comprehensive mesh, material, and animation support.

## Overview

VP-Avatar is a professional-grade Rust library designed to load, parse, and manipulate VRM avatar files created with VRoid Studio. It provides a type-safe, modular interface for working with skeleton structures, meshes, materials, poses, and complete avatar data with extensive serialization and validation capabilities.

## Key Features

- 🎨 **VRM File Support** - Load and manipulate VRM avatar data from VRoid
- 🦴 **Modular Bone System** - Export/import individual bones independently with flexible hierarchy
- 🔗 **Bone Hierarchy** - Parent-child relationships with circular reference detection
- 📦 **Multiple Serialization Formats** - JSON, JSON Lines, Binary (bincode), and human-readable text
- 🎯 **Type-Safe Design** - Leverage Rust's type system for safety and performance
- 🧬 **Extensible Metadata** - Custom properties per bone, mesh, and material
- 🌐 **Mesh Management** - Geometry, UV coordinates, normals, bone weights
- 🎨 **PBR Materials** - Physically-based rendering with metallic, roughness, and emissive properties
- 🎭 **Pose System** - Static poses and keyframe data support
- ✅ **Validation** - Skeleton integrity checking with error reporting
- 📊 **Session Management** - Track operations and edit history
- 📈 **Statistics** - Comprehensive avatar metrics and analysis

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vp-avatar = { git = "https://github.com/ttaichi0505-beep/VP-Avatar" }
```

## Quick Start

### Creating a Complete Avatar

```rust
use vp_avatar::{Avatar, Bone, BoneHierarchy, BoneType, Material, Mesh, Pose};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create avatar
    let mut avatar = Avatar::new("My VRoid Avatar".to_string());
    avatar.set_author("Your Name".to_string());
    
    // Create skeleton
    let mut skeleton = BoneHierarchy::new("Skeleton".to_string());
    
    // Add bones
    let mut armature = Bone::new("Armature".to_string(), None, BoneType::Armature);
    let armature_id = armature.id.clone();
    skeleton.add_bone(armature);
    
    // Add child bones
    let head = Bone::new("Head".to_string(), Some(armature_id.clone()), BoneType::Head);
    skeleton.add_bone(head);
    
    // Attach skeleton to avatar
    avatar.set_skeleton(skeleton);
    
    // Add mesh
    let mut mesh = Mesh::new("Head".to_string());
    mesh.add_vertex(0.0, 0.0, 0.0);
    mesh.add_vertex(1.0, 0.0, 0.0);
    mesh.add_vertex(0.0, 1.0, 0.0);
    mesh.add_face(0, 1, 2);
    avatar.add_mesh(mesh);
    
    // Add material
    let mut material = Material::new("Skin".to_string());
    material.set_base_color(1.0, 0.8, 0.7, 1.0);
    material.set_pbr(0.1, 0.5);
    avatar.add_material(material);
    
    // Add pose
    let pose = Pose::new("Default".to_string());
    avatar.add_pose(pose);
    
    // Get statistics
    let stats = avatar.get_statistics();
    println!("Avatar: {} with {} bones, {} meshes", 
             avatar.name, stats.bone_count, stats.mesh_count);
    
    Ok(())
}
```

### Exporting Avatars

```rust
use vp_avatar::export;

// Export complete avatar to JSON
export::export_avatar_to_json(&avatar, "./avatar.json")?;

// Export to JSON Lines format
export::export_avatar_to_jsonl(&avatar, "./avatar.jsonl")?;

// Export human-readable summary
export::export_avatar_summary(&avatar, "./avatar_summary.txt")?;
```

### Working with Bones

```rust
use vp_avatar::{Bone, BoneType, serialization};

// Create and configure bone
let mut bone = Bone::new("Head".to_string(), None, BoneType::Head);
bone.set_position(0.0, 1.5, 0.0);
bone.set_rotation(0.0, 0.0, 0.0, 1.0);
bone.set_property("material".to_string(), "skin".to_string());

// Export individual bone
serialization::export_bone_to_json(&bone, "./head.json")?;

// Import bone
let loaded_bone = serialization::import_bone_from_json("./head.json")?;

// Export all bones in hierarchy
serialization::export_bones_individually(&skeleton, "./bones")?;

// Import bones from directory
let skeleton = serialization::import_bones_individually("./bones")?;
```

### Mesh and Material Operations

```rust
use vp_avatar::{Mesh, Material, AlphaMode};

// Create mesh with geometry
let mut mesh = Mesh::new("Body".to_string());
mesh.add_vertex(0.0, 0.0, 0.0);
mesh.add_vertex(1.0, 0.0, 0.0);
mesh.add_vertex(0.0, 1.0, 0.0);
mesh.add_face(0, 1, 2);

// Create PBR material
let mut material = Material::new("ShinyMetal".to_string());
material.set_pbr(0.9, 0.2);  // High metallic, low roughness
material.set_emissive(0.1, 0.1, 0.1);
material.set_normal_texture("normal.png".to_string());

// Set alpha transparency
material.alpha_mode = AlphaMode::Blend;

avatar.add_mesh(mesh);
avatar.add_material(material);
```

### Session and Operation Tracking

```rust
use vp_avatar::session::{Session, OperationType};

// Create session
let mut session = Session::new("Edit Session".to_string(), "avatar123".to_string());

// Record operations
session.record_operation(
    OperationType::BoneCreate,
    "Created Head bone".to_string(),
    true,  // reversible
);

println!("Operations: {}", session.operation_count());
```

### Validation

```rust
use vp_avatar::validation;

// Validate skeleton
let report = validation::validate_hierarchy(&skeleton);

if !report.is_valid {
    for error in &report.errors {
        eprintln!("Error: {}", error);
    }
}

println!("Statistics:");
println!("  Total bones: {}", report.stats.bone_count);
println!("  Root bones: {}", report.stats.root_count);
println!("  Leaf bones: {}", report.stats.leaf_count);
```

## Project Structure

```
VP-Avatar/
├── src/
│   ├── lib.rs              # Library root
│   ├── avatar.rs           # Avatar struct and operations
│   ├── bone.rs             # Bone structures and hierarchies
│   ├── mesh.rs             # Mesh geometry and rigging
│   ├── material.rs         # PBR materials
│   ├── pose.rs             # Static poses and keyframes
│   ├── serialization.rs    # Bone import/export
│   ├── export.rs           # Avatar export formats
│   ├── session.rs          # Session management
│   ├── validation.rs       # Skeleton validation
│   ├── utils.rs            # Utility functions
│   ├── error.rs            # Error types
│   └── ...
├── Cargo.toml              # Project configuration
├── README.md               # This file
└── LICENSE                 # MIT License
```

## Modules

### avatar.rs
Core avatar struct with support for skeletons, meshes, materials, and poses.

### bone.rs
Bone and skeleton hierarchy management with parent-child relationships.

### mesh.rs
Mesh geometry with vertices, indices, normals, UV coordinates, and bone weights.

### material.rs
PBR (Physically Based Rendering) material system with textures and properties.

### pose.rs
Static pose and keyframe data storage.

### serialization.rs
Import/export individual bones and hierarchies in JSON and binary formats.

### export.rs
Export complete avatars in JSON, JSON Lines, and human-readable formats.

### session.rs
Operation tracking and session history management.

### validation.rs
Skeleton integrity checking with circular reference detection.

### utils.rs
Helper functions for file operations and data manipulation.

## API Reference

### Avatar Operations

- `Avatar::new()` - Create new avatar
- `Avatar::get_statistics()` - Get avatar metrics
- `Avatar::add_mesh()` / `get_mesh()` - Manage meshes
- `Avatar::add_material()` / `get_material()` - Manage materials
- `Avatar::add_pose()` / `get_pose()` - Manage poses
- `Avatar::set_skeleton()` / `get_skeleton()` - Manage skeleton

### Bone Operations

- `Bone::new()` - Create bone
- `Bone::set_position/rotation/scale()` - Transform bone
- `Bone::set_property()` - Add metadata
- `BoneHierarchy::add_bone()` - Add to hierarchy
- `BoneHierarchy::get_children()` - Get child bones
- `BoneHierarchy::get_bones_by_type()` - Filter by type

### Serialization

- `export_bone_to_json()` - Single bone to JSON
- `import_bone_from_json()` - Load bone from JSON
- `export_bones_individually()` - All bones to separate files
- `import_bones_individually()` - Load from directory
- `export_bone_to_binary()` - Binary format
- `import_bone_from_binary()` - Binary load

### Export

- `export_avatar_to_json()` - Complete avatar to JSON
- `export_avatar_to_jsonl()` - Avatar to JSON Lines
- `export_avatar_summary()` - Human-readable summary

## Bone Types

- `Armature` - Root bone/armature
- `Head` - Head bone
- `Spine` - Spine/neck
- `Torso` - Upper body
- `Arm` - Arm bones
- `Hand` - Hand bones
- `Finger` - Finger bones
- `Leg` - Leg bones
- `Foot` - Foot bones
- `Other` - Other bones

## Alpha Modes

- `Opaque` - Fully opaque material
- `Mask` - Binary transparency (on/off)
- `Blend` - Smooth transparency blending

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
cargo build --release
```

### Documentation

```bash
cargo doc --open
```

## Performance Considerations

- Bones use UUID for unique identification
- Hierarchies efficiently store parent-child relationships
- Validation checks for circular references
- Serialization supports both JSON (readable) and binary (compact) formats

## Limitations

- VRM file parsing not yet implemented (planned for future release)
- Animation/keyframe interpolation not supported
- No built-in mesh optimization

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Areas for contribution:
- VRM file format support (glTF/VRM parsing)
- Additional export formats (FBX, GLTF, etc.)
- Animation system
- Mesh optimization utilities
- Performance improvements

Please feel free to submit a Pull Request.

---

**Status**: Active Development 🚀

**Version**: 0.1.0

**Note**: Individual bones can be exported, modified externally, and re-imported to build custom skeletons dynamically. The modular design allows for flexible avatar composition and manipulation.
