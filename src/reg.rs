use crate::utils::{Response, ResponseData, User};
use base64ct::{Base64, Encoding};
use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    serde::json::Json,
};
use sha2::{Digest, Sha256};
use sqlx::{MySql, Pool};

#[derive(FromForm)]
pub struct LoginForm<'r> {
    phone_num: &'r str,
    password: &'r str,
}
#[post("/login", data = "<form>")]
pub async fn login(
    cookies: &CookieJar<'_>,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<LoginForm<'_>>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let password_hashed = Base64::encode_string(
        Sha256::new()
            .chain_update(form.password)
            .finalize()
            .as_ref(),
    );
    let row = sqlx::query_as!(
        User,
        "SELECT * FROM user WHERE phone_num = ? and password = ?",
        form.phone_num,
        password_hashed
    )
    .fetch_one(conn)
    .await;
    let response: Response;
    match row {
        Ok(result) => {
            cookies.add_private(Cookie::new("user_id", result.id.to_string()));
            cookies.add_private(Cookie::new("token", form.password.to_string()));
            response = Response::new("success", ResponseData::User(result));
        }
        Err(_err) => {
            response = Response::new(
                "failed",
                ResponseData::User(User {
                    id: 0,
                    username: None,
                    password: "0".to_string(),
                    phone_num: "0".to_string(),
                    permission: "user".to_string(),
                    self_sign: None,
                }),
            )
        }
    }
    connection.detach();
    Json(response)
}

#[get("/logout")]
pub async fn logout(_user: User, cookies: &CookieJar<'_>) -> Json<Response> {
    cookies.remove_private("user_id");
    cookies.remove_private("token");
    Json(Response::new(
        "success",
        ResponseData::String("some".to_string()),
    ))
}

#[derive(FromForm)]
pub struct RegForm<'r> {
    phone_num: &'r str,
    password: &'r str,
}
#[post("/register", data = "<form>")]
pub async fn register(
    pool: &rocket::State<Pool<MySql>>,
    form: Form<RegForm<'_>>,
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
                "insert into user (phone_num, password, permission) value (?, ?, ?)",
                form.phone_num,
                password_hashed,
                "user"
            )
            .execute(conn)
            .await;
            match row {
                Ok(_result) => {
                    resp_str = "success";
                    resp_args = "Register Success";
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
