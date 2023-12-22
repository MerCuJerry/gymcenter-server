use crate::utils::{Admin, DeleteForm, Response, User};
use rocket::{form::Form, serde::json::Json, Route};
use sqlx::{MySql, Pool};
mod lib;
use lib::{user_add, user_change, user_delete, user_query, UserAddForm, UserChangeForm};

//用户相关
pub fn get_user_routes() -> Vec<Route> {
    routes![
        user_add_admin,
        user_change_admin,
        user_change_user,
        user_delete_admin,
        user_query_all,
        user_query_one,
    ]
}
#[post("/add", data = "<form>")]
async fn user_add_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<UserAddForm<'_>>,
) -> Json<Response> {
    user_add(pool, form).await
}

#[post("/delete", data = "<form>")]
async fn user_delete_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    user_delete(pool, form).await
}

// 查询用户
#[get("/query")]
async fn user_query_all(_admin: Admin, pool: &rocket::State<Pool<MySql>>) -> Json<Response> {
    user_query(pool, None).await
}
#[get("/query/<id>")]
async fn user_query_one(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    id: i32,
) -> Json<Response> {
    user_query(pool, Some(id)).await
}

//管理员更改用户
#[post("/change", data = "<form>")]
async fn user_change_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<UserChangeForm<'_>>,
) -> Json<Response> {
    user_change(pool, form).await
}
//用户更改用户
#[post("/change", rank = 2, data = "<form>")]
async fn user_change_user(
    user: User,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<UserChangeForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.id = user.id;
    changed_form.permission = user.permission.as_str();
    user_change(pool, changed_form).await
}
