use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::db::{dml, models};
use crate::settings;

pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let users = dml::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    user: web::Json<models::User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: models::User = user.into_inner();

    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let new_user = dml::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
