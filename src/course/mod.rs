use crate::utils::{Admin, Coach, DeleteForm, Response, User};
mod lib;
use lib::{add, change, delete, query, AddForm, ChangeForm};
use rocket::{form::Form, serde::json::Json, Route, State};
use sqlx::{MySql, Pool};

pub fn get_routes() -> Vec<Route> {
    routes![
        add_admin,
        add_coach,
        delete_admin,
        delete_coach,
        query_all,
        query_where_coach,
        change_admin,
        change_coach
    ]
}
//管理员添加课程
#[post("/add", data = "<form>")]
async fn add_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<AddForm<'_>>,
) -> Json<Response> {
    add(pool, form).await
}
//教练添加课程
#[post("/add", rank = 2, data = "<form>")]
async fn add_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<AddForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.coach_id = coach.get_id();
    add(pool, changed_form).await
}

//管理员删除课程
#[post("/delete", data = "<form>")]
async fn delete_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    delete(pool, form, None).await
}
//教练删除课程
#[post("/delete", rank = 2, data = "<form>")]
async fn delete_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    delete(pool, form, Some(coach.get_id())).await
}

#[get("/query")]
async fn query_all(_user: User, pool: &State<Pool<MySql>>) -> Json<Response> {
    query(pool, None).await
}
#[get("/query/<id>")]
async fn query_where_coach(
    _coach: Coach,
    pool: &State<Pool<MySql>>,
    id: i32,
) -> Json<Response> {
    query(pool, Some(id)).await
}

//管理员更改课程
#[post("/change", data = "<form>")]
async fn change_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<ChangeForm<'_>>,
) -> Json<Response> {
    change(pool, form).await
}
//教练更改课程
#[post("/change", rank = 2, data = "<form>")]
async fn change_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<ChangeForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.coach_id = coach.get_id();
    change(pool, changed_form).await
}
