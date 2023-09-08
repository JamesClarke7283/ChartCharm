use sea_orm_migration::prelude::*;
use sea_query::{ColumnDef, Iden};
#[derive(Iden)]
enum DataPoints {
    Table,
    Id,
    Project,
    Data,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Projects {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Settings {
    Table,
    Id,
    ThemeSelected,
}

#[derive(Iden)]
enum Theme {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum ChartKind {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Charts {
    Table,
    Id,
    Name,
    Description,
    Project,
    Kind,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create DataPoints table
        manager
            .create_table(
                Table::create()
                    .table(DataPoints::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataPoints::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DataPoints::Project).integer().not_null())
                    .col(ColumnDef::new(DataPoints::Data).float().not_null())
                    .col(ColumnDef::new(DataPoints::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DataPoints::UpdatedAt).timestamp().not_null())
                    .clone(),
            )
            .await?;

        // Create Projects table
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Projects::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::Description).string().not_null())
                    .col(ColumnDef::new(Projects::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Projects::UpdatedAt).timestamp().not_null())
                    .clone(),
            )
            .await?;

        // Create Settings table
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Settings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Settings::ThemeSelected).integer().not_null())
                    .clone(),
            )
            .await?;

        // Create Theme table
        manager
            .create_table(
                Table::create()
                    .table(Theme::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Theme::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Theme::Name).string().not_null())
                    .clone(),
            )
            .await?;

        // Create ChartKind table
        manager
            .create_table(
                Table::create()
                    .table(ChartKind::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChartKind::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ChartKind::Name).string().not_null())
                    .clone(),
            )
            .await?;

        // Create Charts table
        manager
            .create_table(
                Table::create()
                    .table(Charts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Charts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Charts::Name).string().not_null())
                    .col(ColumnDef::new(Charts::Description).string().not_null())
                    .col(ColumnDef::new(Charts::Project).integer().not_null())
                    .col(ColumnDef::new(Charts::Kind).integer().not_null())
                    .col(ColumnDef::new(Charts::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Charts::UpdatedAt).timestamp().not_null())
                    .clone(),
            )
            .await?;
        // Populate Theme table
        let populate_theme_stmt = Query::insert()
            .into_table(Theme::Table)
            .columns(vec![Theme::Name])
            .values_panic(vec![SimpleExpr::Value(Value::String(Some(Box::new(
                "auto".to_owned(),
            ))))])
            .values_panic(vec![SimpleExpr::Value(Value::String(Some(Box::new(
                "light".to_owned(),
            ))))])
            .values_panic(vec![SimpleExpr::Value(Value::String(Some(Box::new(
                "dark".to_owned(),
            ))))])
            .clone();

        manager.exec_stmt(populate_theme_stmt).await?;

        // Populate Settings table
        let populate_settings_stmt = Query::insert()
            .into_table(Settings::Table)
            .columns(vec![Settings::ThemeSelected])
            .values_panic(vec![SimpleExpr::Value(Value::Int(Some(1)))])
            .clone();

        manager.exec_stmt(populate_settings_stmt).await?;

        // Populate Chart Kind Table
        let populate_chart_kind_stmt = Query::insert()
            .into_table(ChartKind::Table)
            .columns(vec![ChartKind::Name])
            .values_panic(vec![SimpleExpr::Value(Value::String(Some(Box::new(
                "line".to_owned(),
            ))))])
            .clone();
        manager.exec_stmt(populate_chart_kind_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Theme::Table).clone())
            .await?;
        manager
            .drop_table(Table::drop().table(Settings::Table).clone())
            .await?;
        manager
            .drop_table(Table::drop().table(Projects::Table).clone())
            .await?;
        manager
            .drop_table(Table::drop().table(DataPoints::Table).clone())
            .await?;

        Ok(())
    }
}
