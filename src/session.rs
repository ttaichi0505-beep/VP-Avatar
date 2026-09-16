//! Session and context management for avatar operations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents an avatar editing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub id: String,

    /// Session name
    pub name: String,

    /// Avatar ID being edited
    pub avatar_id: String,

    /// Created timestamp
    pub created_at: u64,

    /// Last modified timestamp
    pub modified_at: u64,

    /// Session metadata
    pub metadata: HashMap<String, String>,

    /// Operation history
    pub history: Vec<Operation>,
}

/// Represents a single operation in the session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// Operation ID
    pub id: String,

    /// Operation type
    pub op_type: OperationType,

    /// Operation description
    pub description: String,

    /// Timestamp
    pub timestamp: u64,

    /// Whether operation can be undone
    pub reversible: bool,

    /// Optional context data
    pub context: HashMap<String, String>,
}

/// Types of operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationType {
    /// Bone created
    BoneCreate,
    /// Bone modified
    BoneModify,
    /// Bone deleted
    BoneDelete,
    /// Mesh created
    MeshCreate,
    /// Mesh modified
    MeshModify,
    /// Material created
    MaterialCreate,
    /// Material modified
    MaterialModify,
    /// Pose created
    PoseCreate,
    /// Pose modified
    PoseModify,
    /// Avatar exported
    Export,
    /// Avatar imported
    Import,
    /// Other operation
    Other,
}

impl Session {
    /// Creates a new session
    pub fn new(name: String, avatar_id: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            avatar_id,
            created_at: now,
            modified_at: now,
            metadata: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Records an operation
    pub fn record_operation(
        &mut self,
        op_type: OperationType,
        description: String,
        reversible: bool,
    ) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.history.push(Operation {
            id: uuid::Uuid::new_v4().to_string(),
            op_type,
            description,
            timestamp: now,
            reversible,
            context: HashMap::new(),
        });

        self.modified_at = now;
    }

    /// Gets operation count
    pub fn operation_count(&self) -> usize {
        self.history.len()
    }

    /// Gets reversible operation count
    pub fn reversible_operation_count(&self) -> usize {
        self.history.iter().filter(|op| op.reversible).count()
    }

    /// Sets metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new("Test Session".to_string(), "avatar123".to_string());
        assert_eq!(session.name, "Test Session");
        assert_eq!(session.avatar_id, "avatar123");
        assert_eq!(session.operation_count(), 0);
    }

    #[test]
    fn test_record_operation() {
        let mut session = Session::new("Test".to_string(), "avatar123".to_string());
        session.record_operation(
            OperationType::BoneCreate,
            "Created Head bone".to_string(),
            true,
        );

        assert_eq!(session.operation_count(), 1);
        assert_eq!(session.reversible_operation_count(), 1);
    }
}
