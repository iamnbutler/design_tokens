#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;
#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TokenKind {
    Color,
    Dimension,
    FontFamily,
    FontWeight,
    Duration,
    CubicBezier,
    Number,
    Shadow,
    StrokeStyle,
    Border,
    Transition,
    Typography,
    Gradient,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TokenValue {
    String(String),
    Number(f64),
    Array(Vec<f64>),
    Composite(Box<CompositeValue>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompositeValue {
    color: Option<String>,
    offset_x: Option<String>,
    offset_y: Option<String>,
    blur: Option<String>,
    spread: Option<String>,
    width: Option<String>,
    style: Option<StrokeStyleValue>,
    duration: Option<String>,
    delay: Option<String>,
    timing_function: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StrokeStyleValue {
    String(String),
    Object {
        dash_array: Vec<String>,
        line_cap: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(rename = "$value")]
    value: TokenValue,
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    kind: Option<TokenKind>,
    #[serde(rename = "$description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "$extensions", skip_serializing_if = "Option::is_none")]
    extensions: Option<Map<String, serde_json::Value>>,

    #[serde(flatten)]
    additional_properties: Option<Map<String, serde_json::Value>>,
}
