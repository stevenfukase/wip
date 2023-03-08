use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Role {
    ADMIN,
    GUEST,
    USER,
}
