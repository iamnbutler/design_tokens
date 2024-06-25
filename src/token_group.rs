#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;
#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

use serde::{Deserialize, Serialize};

use crate::token::{Token, TokenKind};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GroupItem {
    Token(Box<Token>),
    Group(TokenGroup),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenGroup {
    #[serde(rename = "$description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    kind: Option<TokenKind>,
    #[serde(rename = "$extensions", skip_serializing_if = "Option::is_none")]
    extensions: Option<Map<String, serde_json::Value>>,
    #[serde(flatten)]
    items: Map<String, GroupItem>,
}
