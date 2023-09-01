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
                    .to_owned(),
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
                    .to_owned(),
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
                    .to_owned(),
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
                    .to_owned(),
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
            .to_owned();

        manager.exec_stmt(populate_theme_stmt).await?;

        // Populate Settings table
        let populate_settings_stmt = Query::insert()
            .into_table(Settings::Table)
            .columns(vec![Settings::ThemeSelected])
            .values_panic(vec![SimpleExpr::Value(Value::Int(Some(1)))])
            .to_owned();

        manager.exec_stmt(populate_settings_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Theme::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DataPoints::Table).to_owned())
            .await?;

        Ok(())
    }
}
