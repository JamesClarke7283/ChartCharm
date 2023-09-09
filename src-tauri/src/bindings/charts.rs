use chartcharm_database::get_connection;
use chartcharm_database::models::charts;
use chartcharm_shared::Chart;
use chartcharm_shared::ChartError;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

#[tauri::command]
pub async fn list_charts() -> Result<Vec<Chart>, ChartError> {
    println!("list_charts function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ChartError::ConnectionError(
                e.to_string(),
                "all".to_string(),
            ));
        }
    };

    println!("Got connection");

    let charts = match charts::Entity::find().all(&conn).await {
        Ok(charts) => charts,
        Err(e) => {
            println!("Failed to get charts: {e:?}");
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    };

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

#[tauri::command]
pub async fn query_chart(id: u16) -> Result<Chart, ChartError> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ChartError::ConnectionError(e.to_string(), id.to_string()));
        }
    };

    println!("Got connection");

    match charts::Entity::find_by_id(id).one(&conn).await {
        Ok(chart) => match chart {
            Some(chart) => {
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
            None => {
                println!("No chart found with id: {id}");
                return Err(ChartError::RetrieveError(id.to_string()));
            }
        },
        Err(e) => {
            println!("Failed to retrieve chart: {e:?}");
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    }
}

#[tauri::command]
pub async fn add_chart(
    name: String,
    description: String,
    project_id: u16,
    kind_id: u8,
) -> Result<(), ChartError> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ChartError::ConnectionError(e.to_string(), name.to_string()));
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

    let chart = match chart.insert(&conn).await {
        Ok(chart) => chart,
        Err(e) => {
            println!("Failed to insert chart: {e:?}");
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    };

    println!("Inserted chart: {chart:?}");

    Ok(())
}

#[tauri::command]
pub async fn delete_chart(id: u16) -> Result<(), ChartError> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ChartError::ConnectionError(e.to_string(), id.to_string()));
        }
    };

    println!("Got connection");

    match charts::Entity::delete_by_id(id).exec(&conn).await {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to delete chart: {e:?}");
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    };

    println!("Deleted chart with id: {id}");

    Ok(())
}

#[tauri::command]
pub async fn update_chart(
    id: u16,
    name: String,
    description: String,
    project_id: u16,
    kind_id: u8,
) -> Result<(), ChartError> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ChartError::ConnectionError(e.to_string(), name.to_string()));
        }
    };

    println!("Got connection");
    let chart = match charts::Entity::find_by_id(id).one(&conn).await {
        Ok(chart) => match chart {
            Some(chart) => chart,
            None => {
                println!("No chart found with id: {id}");
                return Err(ChartError::RetrieveError(id.to_string()));
            }
        },
        Err(e) => {
            println!("Failed to retrieve chart: {e:?}");
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    };
    let mut chart: charts::ActiveModel = chart.into();

    chart.name = Set(name.to_owned());
    chart.description = Set(description.to_owned());
    chart.project = Set(project_id);
    chart.kind = Set(kind_id);
    chart.updated_at = Set(Utc::now());

    match chart.update(&conn).await {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to update chart: {e:?}");
            return Err(ChartError::ConnectionError(e.to_string(), id.to_string()));
        }
    };

    println!("Updated chart with id: {id}");
    Ok(())
}
