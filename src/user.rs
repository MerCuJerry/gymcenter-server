use crate::utils::{Admin, DeleteForm, Response, ResponseData, User};
use rocket::{form::Form, serde::json::Json};
use sqlx::{MySql, Pool};

//用户相关
#[derive(FromForm)]
pub struct UserAddForm<'r> {
    phone_num: &'r str,
    password: &'r str,
}
#[post("/user/add", data = "<form>")]
pub async fn user_add(
    _admin: Admin,
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
                "insert into user (phone_num, password, permission) value (?, ?, ?)",
                form.phone_num,
                form.password,
                "user"
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
pub async fn user_delete(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
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

// 用户
#[post("/user/query")]
pub async fn user_query(_admin: Admin, pool: &rocket::State<Pool<MySql>>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query_as!(User, "SELECT * FROM user")
        .fetch_all(conn)
        .await;
    let response: Response;
    match row {
        Ok(result) => response = Response::new("success", ResponseData::User(result)),
        Err(_err) => {
            response = Response::new("Failed", ResponseData::String("Failed".to_string()));
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
#[post("/user/change", data = "<form>")]
pub async fn user_change(
    _admin: Admin,
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
