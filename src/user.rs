use crate::utils::{Admin, DeleteForm, Response, ResponseData, User};
use rocket::{form::Form, serde::json::Json};
use sqlx::{MySql, Pool};

//用户相关
#[derive(FromForm)]
pub struct UserAddForm<'r> {
    phone_num: &'r str,
    password: &'r str,
    permission: &'r str,
    username: &'r str,
}
#[post("/user/add", data = "<form>")]
pub async fn user_add_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<UserAddForm<'_>>,
) -> Json<Response> {
    user_add(pool, form).await
}
async fn user_add(
    pool: &rocket::State<Pool<MySql>>,
    form: Form<UserAddForm<'_>>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let res = sqlx::query!("select * from user where phone_num = ?", form.phone_num)
        .fetch_one(conn)
        .await;
    let resp_str: &str;
    let resp_args: &str;
    match res {
        Ok(_result) => {
            resp_str = "failed";
            resp_args = "account already exist";
        }
        Err(_err) => {
            let conn = connection.as_mut();
            let row = sqlx::query!(
                "insert into user (phone_num, password, permission, username) value (?, ?, ?, ?)",
                form.phone_num,
                form.password,
                form.permission,
                form.username,
            )
            .execute(conn)
            .await;
            match row {
                Ok(_result) => {
                    resp_str = "success";
                    resp_args = "some";
                }
                Err(_err) => {
                    resp_str = "failed";
                    resp_args = "Wrong account or password";
                }
            }
        }
    }
    connection.detach();
    Json(Response::new(
        resp_str,
        ResponseData::String(resp_args.to_string()),
    ))
}

#[post("/user/delete", data = "<form>")]
pub async fn user_delete_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    user_delete(pool, form).await
}
async fn user_delete(pool: &rocket::State<Pool<MySql>>, form: Form<DeleteForm>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!("DELETE FROM user WHERE id = ?", form.id)
        .execute(conn)
        .await;
    let resp_str: &str;
    let resp_args: &str;
    match row {
        Ok(_result) => {
            resp_str = "success";
            resp_args = "some";
        }
        Err(_err) => {
            resp_str = "failed";
            resp_args = "Wrong while delete user";
        }
    }
    connection.detach();
    Json(Response::new(
        resp_str,
        ResponseData::String(resp_args.to_string()),
    ))
}

// 查询用户
#[get("/user/query")]
pub async fn user_query_all(_admin: Admin, pool: &rocket::State<Pool<MySql>>) -> Json<Response> {
    user_query(pool, None).await
}
#[get("/user/query/<id>")]
pub async fn user_query_one(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    id: i32,
) -> Json<Response> {
    user_query(pool, Some(id)).await
}
async fn user_query(pool: &rocket::State<Pool<MySql>>, id: Option<i32>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn: &mut sqlx::MySqlConnection = connection.as_mut();
    let response: Response;
    if let Some(user_id) = id {
        let row = sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", user_id)
            .fetch_one(conn)
            .await;
        match row {
            Ok(result) => response = Response::new("success", ResponseData::User(result)),
            Err(_err) => {
                response = Response::new("Failed", ResponseData::String("Failed".to_string()));
            }
        }
    } else {
        let row = sqlx::query_as!(User, "SELECT * FROM user")
            .fetch_all(conn)
            .await;
        match row {
            Ok(result) => response = Response::new("success", ResponseData::Users(result)),
            Err(_err) => {
                response = Response::new("Failed", ResponseData::String("Failed".to_string()));
            }
        }
    }
    connection.detach();
    Json(response)
}

#[derive(FromForm)]
pub struct UserChangeForm<'r> {
    id: i32,
    username: &'r str,
    password: &'r str,
    permission: &'r str,
    self_sign: &'r str,
}
//管理员更改用户
#[post("/user/change", data = "<form>")]
pub async fn user_change_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<UserChangeForm<'_>>,
) -> Json<Response> {
    user_change(pool, form).await
}
//用户更改用户
#[post("/user/change", rank = 2, data = "<form>")]
pub async fn user_change_user(
    user: User,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<UserChangeForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.id = user.id;
    changed_form.permission = user.permission.as_str();
    user_change(pool, changed_form).await
}
async fn user_change(
    pool: &rocket::State<Pool<MySql>>,
    form: Form<UserChangeForm<'_>>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!(
        "UPDATE user
        SET username = ?, password = ?, permission = ?, self_sign = ?
        WHERE id = ?",
        form.username,
        form.password,
        form.permission,
        form.self_sign,
        form.id
    )
    .execute(conn)
    .await;
    let resp_str: &str;
    let resp_args: &str;
    match row {
        Ok(_result) => {
            resp_str = "success";
            resp_args = "some";
        }
        Err(_err) => {
            resp_str = "failed";
            resp_args = "Wrong while change user";
        }
    }
    connection.detach();
    Json(Response::new(
        resp_str,
        ResponseData::String(resp_args.to_string()),
    ))
}
