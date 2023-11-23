#[macro_use]
pub extern crate rocket;
pub extern crate sqlx;
use dotenvy::dotenv_override;
use std::env;
use std::error::Error;

mod course;
mod depend;
mod notice;
mod reg;
mod user;
mod utils;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _routes = routes![
        user::user_add_admin,
        user::user_delete_admin,
        user::user_query,
        user::user_change_admin,
        user::user_change_user,
        course::course_add_admin,
        course::course_add_coach,
        course::course_delete_admin,
        course::course_delete_coach,
        course::course_change_admin,
        course::course_change_coach,
        course::course_query,
        notice::notice_add,
        notice::notice_delete,
        notice::notice_query,
        reg::login,
        reg::logout,
        reg::register,
        depend::depend_add_admin,
        depend::depend_add_user,
        depend::depend_delete_admin,
        depend::depend_delete_user,
        depend::depend_query_admin,
        depend::depend_query_user,
    ];
    dotenv_override().expect("Error when reading dotenv");
    let connection_str = env::var("DATABASE_URL").expect("Please check .env file");
    let pool = sqlx::MySqlPool::connect(&connection_str).await?;
    let _rocket = rocket::build()
        .manage(pool)
        .mount("/", _routes)
        .launch()
        .await?;

    Ok(())
}
