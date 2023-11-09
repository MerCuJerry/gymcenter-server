use crate::{course::Course, notice::Notice, user::User};
use rocket::serde::Serialize;

// 可能返回的数据
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ResponseData {
    User(Vec<User>),
    Notice(Vec<Notice>),
    Course(Vec<Course>),
    String(String),
}
// 返回数据
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    result: String,
    data: ResponseData,
}
impl Response {
    pub fn new(result: &str, data: ResponseData) -> Response {
        Response {
            result: String::from(result),
            data,
        }
    }
}

#[derive(FromForm)]
pub struct DeleteForm {
    pub id: i32,
}