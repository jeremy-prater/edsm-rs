use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

//
// Requests
//

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemRequest {
    pub system_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_coordinates: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_permit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_information: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_primary_star: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_hidden: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemsRequest {
    pub system_name: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_coordinates: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_permit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_information: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_primary_star: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_hidden: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_known_coordinates: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_unknown_coordinates: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemsSphere {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_coordinates: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_permit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_information: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_primary_star: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_radius: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemsCube {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_coordinates: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_permit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_information: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_primary_star: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
}

//
// Responses
//

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemResponse {
    pub name: String,
    pub id: Option<i64>,
    pub id64: Option<i64>,
    pub coords: Option<Coords>,
    pub coords_locked: Option<bool>,
    pub require_permit: Option<bool>,
    pub information: Option<Information>,
    pub primary_star: Option<PrimaryStar>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Information {
    pub allegiance: Option<String>,
    pub government: Option<String>,
    pub faction: Option<String>,
    pub faction_state: Option<String>,
    pub population: Option<i64>,
    pub security: Option<String>,
    pub economy: Option<String>,
    pub second_economy: Option<String>,
    pub reserve: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryStar {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub is_scoopable: bool,
}
