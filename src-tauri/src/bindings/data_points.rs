use chartcharm_database::get_connection;
use chartcharm_database::models::data_points;
use chartcharm_shared::data_point::{DataPoint, DataPointError};
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use sea_orm::Set;

#[tauri::command]
pub async fn add_datapoint(project: u16, data: f32) -> Result<(), DataPointError> {
    println!("add_datapoint function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                project.to_string(),
            ));
        }
    };

    println!("Got connection");

    let data_point = data_points::ActiveModel {
        project: Set(project),
        data: Set(data),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    match data_point.insert(&conn).await {
        Ok(data_point) => {
            println!("Added data_point: {data_point:?}");
            Ok(())
        }
        Err(e) => {
            println!("Failed to insert data_point: {e:?}");
            Err(DataPointError::InsertError(e.to_string()))
        }
    }
}

#[tauri::command]
pub async fn list_datapoints(project: u16) -> Result<Vec<DataPoint>, DataPointError> {
    println!("list_datapoints function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                project.to_string(),
            ));
        }
    };

    println!("Got connection");

    let data_points = match data_points::Entity::find()
        .filter(data_points::Column::Project.eq(project))
        .all(&conn)
        .await
    {
        Ok(data_points) => data_points,
        Err(e) => {
            println!("Failed to get data_points: {e:?}");
            return Err(DataPointError::RetrieveError(e.to_string()));
        }
    };

    let data_points = data_points
        .into_iter()
        .map(|data_point| DataPoint {
            id: data_point.id,
            project: data_point.project,
            data: data_point.data,
            created_at: data_point.created_at,
            updated_at: data_point.updated_at,
        })
        .collect();

    println!("Retrieved data_points: {data_points:?}");

    Ok(data_points)
}

#[tauri::command]
pub async fn query_datapoint(project: u16, data: f32) -> Result<DataPoint, DataPointError> {
    println!("query_datapoint function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                project.to_string(),
            ));
        }
    };

    println!("Got connection");

    match data_points::Entity::find()
        .filter(data_points::Column::Id.eq(project))
        .filter(data_points::Column::Data.eq(data))
        .one(&conn)
        .await
    {
        Ok(data_point) => {
            return {
                let data_point = data_point.unwrap();
                let data_point = DataPoint {
                    id: data_point.id,
                    project: data_point.project,
                    data: data_point.data,
                    created_at: data_point.created_at,
                    updated_at: data_point.updated_at,
                };
                println!("Retrieved data_point: {data_point:?}");
                Ok(data_point)
            }
        }
        Err(e) => {
            println!("Failed to get data_point: {e:?}");
            return Err(DataPointError::RetrieveError(e.to_string()));
        }
    };
}

#[tauri::command]
pub async fn update_datapoint(id: u16, new_data: f32) -> Result<(), DataPointError> {
    println!("update_datapoint function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                id.to_string(),
            ));
        }
    };

    println!("Got connection");

    let data_point = match data_points::Entity::find_by_id(id).one(&conn).await {
        Ok(data_point) => match data_point {
            Some(data_point) => data_point,
            None => {
                println!("No data_point found with id: {id}");
                return Err(DataPointError::RetrieveError(id.to_string()));
            }
        },
        Err(e) => {
            println!("Failed to get data_point: {e:?}");
            return Err(DataPointError::RetrieveError(e.to_string()));
        }
    };

    let mut data_point = data_point.into_active_model();

    data_point.data = Set(new_data);
    data_point.updated_at = Set(Utc::now());

    match data_point.update(&conn).await {
        Ok(data_point) => {
            println!("Updated data_point: {data_point:?}");
            Ok(())
        }
        Err(e) => {
            println!("Failed to update data_point: {e:?}");
            Err(DataPointError::UpdateError(e.to_string()))
        }
    }
}

#[tauri::command]
pub async fn delete_datapoint(project: u16, data: f32) -> Result<(), DataPointError> {
    println!("delete_datapoint function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                project.to_string(),
            ));
        }
    };

    println!("Got connection");

    let data_point = match data_points::Entity::find()
        .filter(data_points::Column::Project.eq(project))
        .filter(data_points::Column::Data.eq(data))
        .one(&conn)
        .await
    {
        Ok(data_point) => data_point.unwrap(),
        Err(e) => {
            println!("Failed to get data_point: {e:?}");
            return Err(DataPointError::RetrieveError(e.to_string()));
        }
    };

    match data_point.delete(&conn).await {
        Ok(_) => {
            println!("Deleted data_point with id: {project}");
            Ok(())
        }
        Err(e) => {
            println!("Failed to delete data_point: {e:?}");
            Err(DataPointError::RetrieveError(e.to_string()))
        }
    }
}
