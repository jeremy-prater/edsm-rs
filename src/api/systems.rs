use crate::schema::systems::{
    SystemRequest, SystemResponse, SystemsCube, SystemsRequest, SystemsSphere,
};
use anyhow::Result;

use super::do_api_call;

pub async fn system(request: SystemRequest) -> Result<SystemResponse> {
    Ok(serde_json::from_value(
        do_api_call(
            "https://www.edsm.net/api-v1/system",
            serde_json::to_string(&request)?,
        )
        .await?,
    )?)
}

pub async fn systems(request: SystemsRequest) -> Result<Vec<SystemResponse>> {
    Ok(serde_json::from_value(
        do_api_call(
            "https://www.edsm.net/api-v1/systems",
            serde_json::to_string(&request)?,
        )
        .await?,
    )?)
}

pub async fn systems_sphere(request: SystemsSphere) -> Result<Vec<SystemResponse>> {
    Ok(serde_json::from_value(
        do_api_call(
            "https://www.edsm.net/api-v1/sphere-systems",
            serde_json::to_string(&request)?,
        )
        .await?,
    )?)
}

pub async fn systems_cube(request: SystemsCube) -> Result<Vec<SystemResponse>> {
    Ok(serde_json::from_value(
        do_api_call(
            "https://www.edsm.net/api-v1/cube-systems",
            serde_json::to_string(&request)?,
        )
        .await?,
    )?)
}
