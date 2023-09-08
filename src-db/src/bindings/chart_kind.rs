use crate::get_connection;
use crate::models::chart_kind;
use sea_orm::entity::prelude::*;

pub async fn query_chart_kind(id: u8) -> Result<String, DbErr> {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");

    let chart_kind = match chart_kind::Entity::find_by_id(id).one(&conn).await? {
        Some(chart_kind) => chart_kind,
        None => {
            println!("No chart kind found with id: {id}");
            return Err(DbErr::RecordNotFound(format!(
                "Chart kind with id {id} not found"
            )));
        }
    };

    println!("Retrieved chart kind: {chart_kind:?}");

    Ok(chart_kind.name)
}

pub async fn list_chart_kinds() -> Result<Vec<String>, DbErr> {
    println!("list_chart_kinds function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");

    let chart_kinds = chart_kind::Entity::find().all(&conn).await?;
    let new_chart_kinds = chart_kinds
        .into_iter()
        .map(|chart_kind| chart_kind.name)
        .collect();

    println!("Retrieved chart kinds: {new_chart_kinds:?}");

    Ok(new_chart_kinds)
}
