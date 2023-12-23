use crate::utils::{Admin, DeleteForm, Response, User};
use rocket::{form::Form, serde::json::Json, Route};
use sqlx::{MySql, Pool};
mod lib;
use lib::{add, change, delete, query, AddForm, ChangeForm};

//用户相关
pub fn get_routes() -> Vec<Route> {
    routes![
        add_admin,
        change_admin,
        change_user,
        delete_admin,
        query_all,
        query_one,
    ]
}
#[post("/add", data = "<form>")]
async fn add_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<AddForm<'_>>,
) -> Json<Response> {
    add(pool, form).await
}

#[post("/delete", data = "<form>")]
async fn delete_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    delete(pool, form).await
}

// 查询用户
#[get("/query")]
async fn query_all(_admin: Admin, pool: &rocket::State<Pool<MySql>>) -> Json<Response> {
    query(pool, None).await
}
#[get("/query/<id>")]
async fn query_one(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    id: i32,
) -> Json<Response> {
    query(pool, Some(id)).await
}

//管理员更改用户
#[post("/change", data = "<form>")]
async fn change_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<ChangeForm<'_>>,
) -> Json<Response> {
    change(pool, form).await
}
//用户更改用户
#[post("/change", rank = 2, data = "<form>")]
async fn change_user(
    user: User,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<ChangeForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.id = user.id;
    changed_form.permission = user.permission.as_str();
    change(pool, changed_form).await
}
