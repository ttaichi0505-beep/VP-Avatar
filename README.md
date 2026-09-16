# VP-Avatar

Rust-based VRM avatar handling library for VRoid avatars with modular bone structure.

## Overview

VP-Avatar is a Rust library designed to load, parse, and manipulate VRM avatar files created with VRoid Studio. It provides a type-safe, modular interface for working with skeleton structures, allowing each bone to be exported and imported independently.

## Key Features

- 🎨 **VRM file parsing** - Load and manipulate VRM avatar data
- 🦴 **Modular Bone System** - Export/import individual bones independently
- 🔗 **Flexible Hierarchy** - Build and compose bone structures dynamically
- 📦 **Multiple Formats** - Support for JSON and binary serialization
- 🎯 **Type-Safe** - Leverage Rust's type system for safety
- 🧬 **Bone Metadata** - Extensible custom properties per bone

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vp-avatar = { git = "https://github.com/ttaichi0505-beep/VP-Avatar" }
```

## Quick Start

### Creating an Avatar with Skeleton

```rust
use vp_avatar::{Avatar, Bone, BoneHierarchy, BoneType};

fn main() {
    // Create avatar
    let mut avatar = Avatar::new("My VRoid Avatar".to_string());
    avatar.set_author("Your Name".to_string());
    
    // Create skeleton
    let mut skeleton = BoneHierarchy::new("Skeleton".to_string());
    
    // Add bones
    let mut armature = Bone::new("Armature".to_string(), None, BoneType::Armature);
    let armature_id = armature.id.clone();
    skeleton.add_bone(armature);
    
    // Add child bone
    let head = Bone::new("Head".to_string(), Some(armature_id), BoneType::Head);
    skeleton.add_bone(head);
    
    // Attach skeleton to avatar
    avatar.set_skeleton(skeleton);
    
    println!("Avatar: {} with {} bones", avatar.name, 
             avatar.get_skeleton().unwrap().bone_count());
}
```

### Exporting Bones Individually

```rust
use vp_avatar::serialization;

// Export all bones to individual JSON files
serialization::export_bones_individually(&skeleton, "./bones")?;

// Each bone is saved as: bones/{bone_id}.json
// Hierarchy metadata is saved as: bones/_hierarchy.json
```

### Importing Bones

```rust
use vp_avatar::serialization;

// Import bones from directory
let skeleton = serialization::import_bones_individually("./bones")?;
```

### Working with Individual Bones

```rust
use vp_avatar::{Bone, BoneType};
use vp_avatar::serialization;

// Create a bone
let mut bone = Bone::new("Head".to_string(), None, BoneType::Head);
bone.set_position(0.0, 1.5, 0.0);
bone.set_rotation(0.0, 0.0, 0.0, 1.0);
bone.set_property("material".to_string(), "skin".to_string());

// Export to JSON
serialization::export_bone_to_json(&bone, "./head_bone.json")?;

// Import from JSON
let loaded_bone = serialization::import_bone_from_json("./head_bone.json")?;
```

## Project Structure

```
VP-Avatar/
├── src/
│   ├── lib.rs              # Library root
│   ├── avatar.rs           # Avatar struct and utilities
│   ├── bone.rs             # Bone structures and hierarchies
│   ├── serialization.rs    # Import/export functionality
│   ├── error.rs            # Error types
│   └── ...
├── Cargo.toml              # Project configuration
└── README.md               # This file
```

## API Highlights

### Bone Operations

- `Bone::new()` - Create a new bone
- `Bone::set_position()` - Set bone position
- `Bone::set_rotation()` - Set bone rotation (quaternion)
- `Bone::set_scale()` - Set bone scale
- `Bone::set_property()` - Add custom metadata

### Hierarchy Management

- `BoneHierarchy::new()` - Create a skeleton
- `BoneHierarchy::add_bone()` - Add a bone to hierarchy
- `BoneHierarchy::get_bone()` - Retrieve a bone
- `BoneHierarchy::get_children()` - Get child bones
- `BoneHierarchy::get_bones_by_type()` - Filter bones by type

### Serialization

- `export_bone_to_json()` - Save single bone to JSON
- `import_bone_from_json()` - Load single bone from JSON
- `export_bones_individually()` - Save all bones as separate files
- `import_bones_individually()` - Load bones from directory
- `export_bone_to_binary()` - Binary format support
- `import_bone_from_binary()` - Binary format support

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

## Bone Types

- `Armature` - Root bone/armature
- `Head` - Head and related
- `Spine` - Neck and spine
- `Torso` - Upper body
- `Arm` - Arm bones
- `Hand` - Hand bones
- `Finger` - Finger bones
- `Leg` - Leg bones
- `Foot` - Foot bones
- `Other` - Other bones

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

**Status**: Early Development 🚀

**Note**: Individual bones can be exported, modified, and re-imported to build custom skeletons dynamically.
