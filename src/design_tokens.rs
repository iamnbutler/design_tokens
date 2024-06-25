#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;
#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

use serde::{Deserialize, Serialize};

use crate::token_group::TokenGroup;

#[derive(Serialize, Deserialize, Debug)]
pub struct DesignTokens {
    #[serde(flatten)]
    groups: Map<String, TokenGroup>,
}
