#[macro_use]
pub extern crate rocket;
pub extern crate sqlx;
use dotenvy::dotenv_override;
use std::env;
use std::error::Error;

mod course;
mod notice;
mod reg;
mod user;
mod utils;

//
use crate::reg::login;
use crate::reg::logout;
use crate::reg::register;
//用户增删改查
use crate::user::user_add;
use crate::user::user_change;
use crate::user::user_delete;
use crate::user::user_query;
//课程增删改查
use crate::course::course_add;
use crate::course::course_change;
use crate::course::course_delete;
use crate::course::course_query;
//通知增删改查
use crate::notice::notice_add;
use crate::notice::notice_delete;
use crate::notice::notice_query;
//主函数
#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _routes = routes![
        user_add,
        user_delete,
        user_query,
        user_change,
        course_add,
        course_delete,
        course_query,
        course_change,
        notice_add,
        notice_delete,
        notice_query,
        login,
        logout,
        register
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
