use std::env;
use self::sea_orm::{Database, DbErr};
use sea_orm_migration::prelude::*;

use crate::data::migrations::Migrator;

pub async fn connect_database() -> Result<(), DbErr> {
    let db = Database::connect(env::var("DATABASE_URL").expect("No database URL in environment.")).await?;
    Migrator::refresh(&db).await?;

    Ok(())
}
