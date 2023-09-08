use crate::get_connection;
use crate::models::charts;
use chartcharm_shared::Chart;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

pub async fn list_charts() -> Result<Vec<Chart>, DbErr> {
    println!("list_charts function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");

    let charts = charts::Entity::find().all(&conn).await?;

    let charts = charts
        .into_iter()
        .map(|chart| Chart {
            id: chart.id,
            name: chart.name,
            description: chart.description,
            project: chart.project,
            kind: chart.kind,
            created_at: chart.created_at,
            updated_at: chart.updated_at,
        })
        .collect();

    println!("Retrieved charts: {charts:?}");

    Ok(charts)
}

pub async fn query_chart(id: u16) -> Result<Chart, DbErr> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");

    let chart = match charts::Entity::find_by_id(id).one(&conn).await? {
        Some(chart) => chart,
        None => {
            println!("No chart found with id: {id}");
            return Err(DbErr::RecordNotFound(format!(
                "Chart with id {id} not found"
            )));
        }
    };

    println!("Retrieved chart: {chart:?}");

    Ok(Chart {
        id: chart.id,
        name: chart.name,
        description: chart.description,
        project: chart.project,
        kind: chart.kind,
        created_at: chart.created_at,
        updated_at: chart.updated_at,
    })
}

pub async fn add_chart(
    name: String,
    description: String,
    project_id: u16,
    kind_id: u8,
) -> Result<(), DbErr> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");

    let chart = charts::ActiveModel {
        name: Set(name.to_owned()),
        description: Set(description.to_owned()),
        project: Set(project_id),
        kind: Set(kind_id),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    let chart = chart.insert(&conn).await?;

    println!("Inserted chart: {chart:?}");

    Ok(())
}

pub async fn delete_chart(id: u16) -> Result<(), DbErr> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");

    charts::Entity::delete_by_id(id).exec(&conn).await?;

    println!("Deleted chart with id: {id}");

    Ok(())
}

pub async fn update_chart(
    id: u16,
    name: String,
    description: String,
    project_id: u16,
    kind_id: u8,
) -> Result<(), DbErr> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");
    let chart = match charts::Entity::find_by_id(id).one(&conn).await? {
        Some(chart) => chart,
        None => {
            println!("No chart found with id: {id}");
            return Err(DbErr::RecordNotFound(format!(
                "Chart with id {id} not found"
            )));
        }
    };
    let mut chart: charts::ActiveModel = chart.into();

    chart.name = Set(name.to_owned());
    chart.description = Set(description.to_owned());
    chart.project = Set(project_id);
    chart.kind = Set(kind_id);
    chart.updated_at = Set(Utc::now());

    chart.update(&conn).await?;

    println!("Updated chart with id: {id}");
    Ok(())
}
