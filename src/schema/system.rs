use serde::{Deserialize, Serialize};

//
// Requests
//

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_id: Option<u32>,
}


//
// Responses
//

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodiesResponse {
    pub id: i64,
    pub id64: Option<i64>,
    pub name: String,
    pub url: String,
    pub body_count: Option<i64>,
    pub bodies: Option<Vec<Body>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub id: i64,
    pub id64: Option<i64>,
    pub body_id: Option<i64>,
    pub name: String,
    pub discovery: Option<Discovery>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub sub_type: Option<String>,
    pub parents: Option<Vec<Parent>>,
    pub distance_to_arrival: Option<f64>,
    pub is_main_star: Option<bool>,
    pub is_scoopable: Option<bool>,
    pub age: Option<f64>,
    pub spectral_class: Option<String>,
    pub luminosity: Option<String>,
    pub absolute_magnitude: Option<f64>,
    pub solar_masses: Option<f64>,
    pub solar_radius: Option<f64>,
    pub surface_temperature: Option<i64>,
    pub orbital_period: Option<f64>,
    pub semi_major_axis: Option<f64>,
    pub orbital_eccentricity: Option<f64>,
    pub orbital_inclination: Option<f64>,
    pub arg_of_periapsis: Option<f64>,
    pub rotational_period: Option<f64>,
    pub rotational_period_tidally_locked: Option<bool>,
    pub axial_tilt: Option<f64>,
    pub update_time: Option<String>,
    pub is_landable: Option<bool>,
    pub gravity: Option<f64>,
    pub earth_masses: Option<f64>,
    pub radius: Option<f64>,
    pub surface_pressure: Option<f64>,
    pub volcanism_type: Option<String>,
    pub atmosphere_type: Option<String>,
    pub atmosphere_composition: Option<AtmosphereComposition>,
    pub solid_composition: Option<SolidComposition>,
    pub terraforming_state: Option<String>,
    #[serde(default)]
    pub rings: Option<Vec<Ring>>,
    pub reserve_level: Option<String>,
    pub materials: Option<Materials>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discovery {
    pub commander: String,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "Null")]
    pub null: Option<i64>,
    #[serde(rename = "Star")]
    pub star: Option<i64>,
    #[serde(rename = "Planet")]
    pub planet: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtmosphereComposition {
    #[serde(rename = "Helium")]
    pub helium: Option<f64>,
    #[serde(rename = "Hydrogen")]
    pub hydrogen: Option<f64>,
    #[serde(rename = "Neon")]
    pub neon: Option<f64>,
    #[serde(rename = "Methane")]
    pub methane: Option<f64>,
    #[serde(rename = "Carbon dioxide")]
    pub carbon_dioxide: Option<f64>,
    #[serde(rename = "Ammonia")]
    pub ammonia: Option<f64>,
    #[serde(rename = "Sulphur dioxide")]
    pub sulphur_dioxide: Option<f64>,
    #[serde(rename = "Silicates")]
    pub silicates: Option<f64>,
    #[serde(rename = "Oxygen")]
    pub oxygen: Option<f64>,
    #[serde(rename = "Nitrogen")]
    pub nitrogen: Option<f64>,
    #[serde(rename = "Water")]
    pub water: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolidComposition {
    #[serde(rename = "Ice")]
    pub ice: Option<f64>,
    #[serde(rename = "Rock")]
    pub rock: Option<f64>,
    #[serde(rename = "Metal")]
    pub metal: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ring {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub mass: Option<f64>,
    pub inner_radius: Option<f64>,
    pub outer_radius: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Materials {
    #[serde(rename = "Sulphur")]
    pub sulphur: Option<f64>,
    #[serde(rename = "Carbon")]
    pub carbon: Option<f64>,
    #[serde(rename = "Phosphorus")]
    pub phosphorus: Option<f64>,
    #[serde(rename = "Iron")]
    pub iron: Option<f64>,
    #[serde(rename = "Nickel")]
    pub nickel: Option<f64>,
    #[serde(rename = "Manganese")]
    pub manganese: Option<f64>,
    #[serde(rename = "Germanium")]
    pub germanium: Option<f64>,
    #[serde(rename = "Vanadium")]
    pub vanadium: Option<f64>,
    #[serde(rename = "Niobium")]
    pub niobium: Option<f64>,
    #[serde(rename = "Molybdenum")]
    pub molybdenum: Option<f64>,
    #[serde(rename = "Ruthenium")]
    pub ruthenium: Option<f64>,
    #[serde(rename = "Tellurium")]
    pub tellurium: Option<f64>,
    #[serde(rename = "Tungsten")]
    pub tungsten: Option<f64>,
    #[serde(rename = "Mercury")]
    pub mercury: Option<f64>,
    #[serde(rename = "Zinc")]
    pub zinc: Option<f64>,
    #[serde(rename = "Arsenic")]
    pub arsenic: Option<f64>,
    #[serde(rename = "Cadmium")]
    pub cadmium: Option<f64>,
    #[serde(rename = "Yttrium")]
    pub yttrium: Option<f64>,
    #[serde(rename = "Tin")]
    pub tin: Option<f64>,
    #[serde(rename = "Chromium")]
    pub chromium: Option<f64>,
    #[serde(rename = "Antimony")]
    pub antimony: Option<f64>,
    #[serde(rename = "Zirconium")]
    pub zirconium: Option<f64>,
    #[serde(rename = "Polonium")]
    pub polonium: Option<f64>,
    #[serde(rename = "Technetium")]
    pub technetium: Option<f64>,
    #[serde(rename = "Selenium")]
    pub selenium: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedValueResponse {
    pub id: i64,
    pub id64: Option<i64>,
    pub name: String,
    pub url: String,
    pub estimated_value: f64,
    pub estimated_value_mapped: f64,
    pub valuable_bodies: Option<Vec<ValuableBody>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValuableBody {
    pub body_id: Option<i64>,
    pub body_name: String,
    pub distance: f64,
    pub value_max: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationResponse {
    pub id: i64,
    pub id64: Option<i64>,
    pub name: String,
    pub url: String,
    pub stations: Option<Vec<Station>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub id: i64,
    pub market_id: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub name: String,
    pub body: Option<StationBody>,
    pub distance_to_arrival: Option<f64>,
    pub allegiance: Option<String>,
    pub government: Option<String>,
    pub economy: Option<String>,
    pub second_economy: Option<String>,
    pub have_market: Option<bool>,
    pub have_shipyard: Option<bool>,
    pub have_outfitting: Option<bool>,
    pub other_services: Option<Vec<String>>,
    pub controlling_faction: Option<ControllingFaction>,
    pub update_time: UpdateTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationBody {
    pub id: i64,
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllingFaction {
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTime {
    pub information: Option<String>,
    pub market: Option<String>,
    pub shipyard: Option<String>,
    pub outfitting: Option<String>,
}
