use crate::utils::{DeleteForm, Response, ResponseData, User};
use rocket::{State, form::Form, serde::json::Json};
use sha2::{Sha256, Digest};
use sqlx::{MySql, Pool};
use base64ct::{Base64, Encoding};

#[derive(FromForm)]
pub struct AddForm<'r> {
    phone_num: &'r str,
    password: &'r str,
    permission: &'r str,
    username: &'r str,
}

#[derive(FromForm)]
pub struct ChangeForm<'r> {
    pub id: i32,
    username: &'r str,
    password: &'r str,
    pub permission: &'r str,
    self_sign: &'r str,
}
pub async fn add(
    pool: &State<Pool<MySql>>,
    form: Form<AddForm<'_>>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let res = sqlx::query!("select * from user where phone_num = ?", form.phone_num)
        .fetch_one(conn)
        .await;
    let resp_str: &str;
    let resp_args: &str;
    let password_hashed = Base64::encode_string(
        Sha256::new()
            .chain_update(form.password)
            .finalize()
            .as_ref(),
    );
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
                password_hashed,
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

pub async fn delete(pool: &State<Pool<MySql>>, form: Form<DeleteForm>) -> Json<Response> {
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


pub async fn query(pool: &State<Pool<MySql>>, id: Option<i32>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn: &mut sqlx::MySqlConnection = connection.as_mut();
    let response: Response;
    if let Some(id) = id {
        let row = sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", id)
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

pub async fn change(
    pool: &State<Pool<MySql>>,
    form: Form<ChangeForm<'_>>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let password_hashed = Base64::encode_string(
        Sha256::new()
            .chain_update(form.password)
            .finalize()
            .as_ref(),
    );
    let row = sqlx::query!(
        "UPDATE user
        SET username = ?, password = ?, permission = ?, self_sign = ?
        WHERE id = ?",
        form.username,
        password_hashed,
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
