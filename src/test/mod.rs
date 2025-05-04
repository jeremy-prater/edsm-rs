use crate::schema::{
    system::SystemInfoRequest,
    systems::{SystemRequest, SystemsCube, SystemsRequest, SystemsSphere},
};
use anyhow::Result;

#[tokio::test]
async fn systems_system() -> Result<()> {
    let _ = crate::api::systems::system(SystemRequest {
        system_name: "Maia".to_string(),
        show_id: Some(1),
        show_coordinates: Some(1),
        show_permit: Some(1),
        show_information: Some(1),
        show_primary_star: Some(1),
        ..Default::default()
    })
    .await?;

    let _ = crate::api::systems::system(SystemRequest {
        system_name: "Maia".to_string(),
        ..Default::default()
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn systems_systems() -> Result<()> {
    let _ = crate::api::systems::systems(SystemsRequest {
        system_name: vec!["Maia".to_string(), "Sol".to_string(), "Merope".to_string()],
        show_id: Some(1),
        show_coordinates: Some(1),
        show_permit: Some(1),
        show_information: Some(1),
        show_primary_star: Some(1),
        ..Default::default()
    })
    .await?;

    let _ = crate::api::systems::systems(SystemsRequest {
        system_name: vec!["Maia".to_string(), "Sol".to_string(), "Merope".to_string()],
        ..Default::default()
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn systems_systems_sphere() -> Result<()> {
    let _ = crate::api::systems::systems_sphere(SystemsSphere {
        system_name: Some("Maia".to_string()),
        show_id: Some(1),
        show_coordinates: Some(1),
        show_permit: Some(1),
        show_information: Some(1),
        show_primary_star: Some(1),
        radius: Some(10),
        ..Default::default()
    })
    .await?;

    let _ = crate::api::systems::systems_sphere(SystemsSphere {
        system_name: Some("Maia".to_string()),
        radius: Some(10),
        ..Default::default()
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn systems_systems_cube() -> Result<()> {
    let _ = crate::api::systems::systems_cube(SystemsCube {
        system_name: Some("Maia".to_string()),
        show_id: Some(1),
        show_coordinates: Some(1),
        show_permit: Some(1),
        show_information: Some(1),
        show_primary_star: Some(1),
        size: Some(10),
        ..Default::default()
    })
    .await?;

    let _ = crate::api::systems::systems_cube(SystemsCube {
        system_name: Some("Maia".to_string()),
        size: Some(10),
        ..Default::default()
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn system_bodies() -> Result<()> {
    let _ = crate::api::system::bodies(SystemInfoRequest {
        system_name: Some("Maia".to_string()),
        ..Default::default()
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn system_estimated_value() -> Result<()> {
    let _ = crate::api::system::estimated_value(SystemInfoRequest {
        system_name: Some("Sol".to_string()),
        ..Default::default()
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn system_stations() -> Result<()> {
    let _ = crate::api::system::stations(SystemInfoRequest {
        system_name: Some("Sol".to_string()),
        ..Default::default()
    })
    .await?;

    Ok(())
}
