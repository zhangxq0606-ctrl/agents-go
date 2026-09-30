use serde::{Deserialize, Serialize};

/// A directory remembered by this server after its workspace was closed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ProjectInfo {
    pub cwd: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ProjectTarget {
    pub cwd: String,
}
