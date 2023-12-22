#[macro_use]
pub extern crate rocket;
pub extern crate sqlx;
use dotenvy::dotenv_override;
use std::{env, error::Error};

mod course;
mod depend;
mod notice;
mod reg;
mod user;
mod utils;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _routes = routes![reg::login, reg::logout, reg::register,];
    dotenv_override().expect("Error when reading dotenv");
    let connection_str = env::var("DATABASE_URL").expect("Please check .env file");
    let pool = sqlx::MySqlPool::connect(&connection_str).await?;
    let _rocket = rocket::build()
        .manage(pool)
        .mount("/course", course::get_course_routes())
        .mount("/depend", depend::get_depend_routes())
        .mount("/notice", notice::get_notice_routes())
        .mount("/user", user::get_user_routes())
        .mount("/", _routes)
        .launch()
        .await?;
    Ok(())
}
