use crate::utils::{Admin, Coach, DeleteForm, Response, User};
mod lib;
use lib::{add, delete, query, QueryType, AddForm};
use rocket::{State, Route, form::Form, serde::json::Json};
use sqlx::{MySql, Pool};

pub fn get_routes() -> Vec<Route> {
    routes![
        add_admin,
        add_user,
        delete_admin,
        delete_user,
        query_coach_user,
        query_course_user,
        query_user_coach
    ]
}
// 管理员添加课程学员
#[post("/add", data = "<form>")]
async fn add_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<AddForm>,
) -> Json<Response> {
    add(pool, form).await
}
// 用户添加课程学员
#[post("/add", rank = 2, data = "<form>")]
async fn add_user(
    user: User,
    pool: &State<Pool<MySql>>,
    form: Form<AddForm>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.user_id = user.id;
    add(pool, changed_form).await
}

// 管理员删除课程学员
#[post("/delete", data = "<form>")]
async fn delete_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    delete(pool, form, None).await
}
// 用户删除课程学员
#[post("/delete", rank = 2, data = "<form>")]
async fn delete_user(
    user: User,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    delete(pool, form, Some(user.id)).await
}

// 用户查询课程学员
#[get("/query/course")]
async fn query_course_user(
    user: User,
    pool: &State<Pool<MySql>>,
) -> Json<Response> {
    query(pool, QueryType::Course, user.id).await
}
// 用户查询教练
#[get("/query/coach")]
async fn query_coach_user(
    user: User,
    pool: &State<Pool<MySql>>,
) -> Json<Response> {
    query(pool, QueryType::Coach, user.id).await
}
// 教练查询学员
#[get("/query/user/<course_id>")]
async fn query_user_coach(
    _coach: Coach,
    pool: &State<Pool<MySql>>,
    course_id: i32,
) -> Json<Response> {
    query(pool, QueryType::User, course_id).await
}
