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
use crate::course::course_add_admin;
use crate::course::course_add_coach;
use crate::course::course_change_admin;
use crate::course::course_change_coach;
use crate::course::course_delete_admin;
use crate::course::course_delete_coach;
use crate::course::course_query;
//通知增删改查
use crate::notice::notice_add;
use crate::notice::notice_delete;
use crate::notice::notice_query;
//
use crate::depend::depend_add;
use crate::depend::depend_delete;
use crate::depend::depend_query;
//主函数
#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _routes = routes![
        user_add,
        user_delete,
        user_query,
        user_change,
        course_add_admin,
        course_add_coach,
        course_delete_admin,
        course_delete_coach,
        course_change_admin,
        course_change_coach,
        course_query,
        notice_add,
        notice_delete,
        notice_query,
        login,
        logout,
        register,
        depend_add,
        depend_delete,
        depend_query,
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
