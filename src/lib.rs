//! VP-Avatar: Rust-based VRM avatar handling library for VRoid avatars
//!
//! This library provides utilities for loading, parsing, and manipulating VRM avatar files
//! created with VRoid Studio. Bones can be exported and imported independently, and meshes,
//! materials, poses, and entire avatars can be managed and serialized in multiple formats.

pub mod avatar;
pub mod bone;
pub mod error;
pub mod export;
pub mod material;
pub mod mesh;
pub mod pose;
pub mod serialization;
pub mod session;
pub mod utils;
pub mod validation;

pub use avatar::{Avatar, AvatarStatistics};
pub use bone::{Bone, BoneHierarchy, BoneTransform, BoneType};
pub use error::{AvatarError, AvatarResult};
pub use export::{export_avatar_to_json, export_avatar_to_jsonl, export_avatar_summary};
pub use material::{AlphaMode, Material};
pub use mesh::Mesh;
pub use pose::Pose;
pub use session::{Operation, OperationType, Session};
pub use validation::validate_hierarchy;

/// Version of the VP-Avatar library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
