use crate::utils::{Admin, User};
use crate::utils::{DeleteForm, Response, ResponseData};
use rocket::{
    form::Form,
    serde::{json::Json, Serialize},
};
use sqlx::{MySql, Pool};

// 通知
#[derive(FromForm)]
pub struct NoticeAddForm {
    notice_content: String,
}
#[post("/notice/add", data = "<form>")]
pub async fn notice_add(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<NoticeAddForm>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!(
        "INSERT INTO notice (notice_content) VALUE (?)",
        form.notice_content
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
            resp_args = "failed add notice";
        }
    }
    connection.detach();
    Json(Response::new(
        resp_str,
        ResponseData::String(resp_args.to_string()),
    ))
}

//删除公告
#[post("/notice/delete", data = "<form>")]
pub async fn notice_delete(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!("DELETE FROM notice WHERE id = ?", form.id)
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
            resp_args = "Wrong while delete notice";
        }
    }
    connection.detach();
    Json(Response::new(
        resp_str,
        ResponseData::String(resp_args.to_string()),
    ))
}

//查询公告
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notice {
    id: i32,
    notice_content: String,
}
#[post("/notice/query")]
pub async fn notice_query(_user: User, pool: &rocket::State<Pool<MySql>>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query_as!(Notice, "SELECT * FROM notice")
        .fetch_all(conn)
        .await;
    let response: Response;
    match row {
        Ok(result) => response = Response::new("success", ResponseData::Notice(result)),
        Err(_err) => {
            response = Response::new("Failed", ResponseData::String("Failed".to_string()));
        }
    }
    connection.detach();
    Json(response)
}
