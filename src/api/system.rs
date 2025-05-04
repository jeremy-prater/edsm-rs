use anyhow::Result;

use crate::schema::system::{BodiesResponse, EstimatedValueResponse, StationResponse, SystemInfoRequest};

use super::do_api_call;

pub async fn bodies(request: SystemInfoRequest) -> Result<BodiesResponse> {
    Ok(serde_json::from_value(
        do_api_call(
            "https://www.edsm.net/api-system-v1/bodies",
            serde_json::to_string(&request)?,
        )
        .await?,
    )?)
}

pub async fn estimated_value(request: SystemInfoRequest) -> Result<EstimatedValueResponse> {
    Ok(serde_json::from_value(
        do_api_call(
            "https://www.edsm.net/api-system-v1/estimated-value",
            serde_json::to_string(&request)?,
        )
        .await?,
    )?)
}

pub async fn stations(request: SystemInfoRequest) -> Result<StationResponse> {
    Ok(serde_json::from_value(
        do_api_call(
            "https://www.edsm.net/api-system-v1/stations",
            serde_json::to_string(&request)?,
        )
        .await?,
    )?)
}