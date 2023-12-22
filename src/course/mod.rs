use crate::utils::{Admin, Coach, DeleteForm, Response, User};
mod lib;
use lib::{
    course_add, course_change, course_delete, course_query, CourseAddForm, CourseChangeForm,
};
use rocket::{form::Form, serde::json::Json, Route, State};
use sqlx::{MySql, Pool};

pub fn get_course_routes() -> Vec<Route> {
    routes![
        course_add_admin,
        course_add_coach,
        course_delete_admin,
        course_delete_coach,
        course_query_all,
        course_query_where_coach,
        course_change_admin,
        course_change_coach
    ]
}
//管理员添加课程
#[post("/add", data = "<form>")]
async fn course_add_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<CourseAddForm<'_>>,
) -> Json<Response> {
    course_add(pool, form).await
}
//教练添加课程
#[post("/add", rank = 2, data = "<form>")]
async fn course_add_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<CourseAddForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.coach_id = coach.get_id();
    course_add(pool, changed_form).await
}

//管理员删除课程
#[post("/delete", data = "<form>")]
async fn course_delete_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    course_delete(pool, form, None).await
}
//教练删除课程
#[post("/delete", rank = 2, data = "<form>")]
async fn course_delete_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    course_delete(pool, form, Some(coach.get_id())).await
}

#[get("/query")]
async fn course_query_all(_user: User, pool: &State<Pool<MySql>>) -> Json<Response> {
    course_query(pool, None).await
}
#[get("/query/<id>")]
async fn course_query_where_coach(
    _coach: Coach,
    pool: &State<Pool<MySql>>,
    id: i32,
) -> Json<Response> {
    course_query(pool, Some(id)).await
}

//管理员更改课程
#[post("/change", data = "<form>")]
async fn course_change_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<CourseChangeForm<'_>>,
) -> Json<Response> {
    course_change(pool, form).await
}
//教练更改课程
#[post("/change", rank = 2, data = "<form>")]
async fn course_change_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<CourseChangeForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.coach_id = coach.get_id();
    course_change(pool, changed_form).await
}
