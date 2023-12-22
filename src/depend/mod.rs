use crate::utils::{Admin, Coach, DeleteForm, Response, User};
mod lib;
use lib::{depend_add, depend_delete, depend_query, QueryType, DependAddForm};
use rocket::{State, Route, form::Form, serde::json::Json};
use sqlx::{MySql, Pool};

pub fn get_depend_routes() -> Vec<Route> {
    routes![
        depend_add_admin,
        depend_add_user,
        depend_delete_admin,
        depend_delete_user,
        depend_query_coach_user,
        depend_query_course_user,
        depend_query_user_coach
    ]
}
// 管理员添加课程学员
#[post("/add", data = "<form>")]
async fn depend_add_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<DependAddForm>,
) -> Json<Response> {
    depend_add(pool, form).await
}
// 用户添加课程学员
#[post("/add", rank = 2, data = "<form>")]
async fn depend_add_user(
    user: User,
    pool: &State<Pool<MySql>>,
    form: Form<DependAddForm>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.user_id = user.id;
    depend_add(pool, changed_form).await
}

// 管理员删除课程学员
#[post("/delete", data = "<form>")]
async fn depend_delete_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    depend_delete(pool, form, None).await
}
// 用户删除课程学员
#[post("/delete", rank = 2, data = "<form>")]
async fn depend_delete_user(
    user: User,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    depend_delete(pool, form, Some(user.id)).await
}

// 用户查询课程学员
#[get("/query/course")]
async fn depend_query_course_user(
    user: User,
    pool: &State<Pool<MySql>>,
) -> Json<Response> {
    depend_query(pool, QueryType::Course, user.id).await
}
// 用户查询教练
#[get("/query/coach")]
async fn depend_query_coach_user(
    user: User,
    pool: &State<Pool<MySql>>,
) -> Json<Response> {
    depend_query(pool, QueryType::Coach, user.id).await
}
// 教练查询学员
#[get("/query/user/<course_id>")]
async fn depend_query_user_coach(
    _coach: Coach,
    pool: &State<Pool<MySql>>,
    course_id: i32,
) -> Json<Response> {
    depend_query(pool, QueryType::User, course_id).await
}
