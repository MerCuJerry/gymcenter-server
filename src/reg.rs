use crate::utils::User;
use crate::utils::{Response, ResponseData};
use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    serde::json::Json,
};
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
    let row = sqlx::query!(
        "SELECT * FROM user WHERE phone_num = ? and password = ?",
        form.phone_num,
        form.password
    )
    .fetch_one(conn)
    .await;
    let resp_str: &str;
    let resp_args: String;
    match row {
        Ok(result) => {
            resp_str = "success";
            cookies.add_private(Cookie::new("user_id", result.id.to_string()));
            cookies.add_private(Cookie::new("token", result.password.to_string()));
            resp_args = "placeholder".to_string();
        }
        Err(_err) => {
            resp_str = "failed";
            resp_args = "Wrong account or password".to_string();
        }
    }
    connection.detach();
    Json(Response::new(resp_str, ResponseData::String(resp_args)))
}

#[get("/logout")]
pub async fn logout(
    _user: User,
    cookies: &CookieJar<'_>,
) -> Json<Response> {
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
