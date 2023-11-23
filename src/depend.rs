use crate::utils::{Admin, DeleteForm, Response, ResponseData, User};
use rocket::{
    form::Form,
    serde::{json::Json, Serialize},
};
use sqlx::{MySql, Pool};

#[derive(FromForm)]
pub struct DependAddForm {
    course_id: i32,
    user_id: i32,
}
// 管理员添加课程学员
#[post("/depend/add", data = "<form>")]
pub async fn depend_add_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DependAddForm>,
) -> Json<Response> {
    depend_add(pool, form).await
}
// 用户添加课程学员
#[post("/depend/add", rank = 2, data = "<form>")]
pub async fn depend_add_user(
    user: User,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DependAddForm>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.user_id = user.id;
    depend_add(pool, changed_form).await
}
async fn depend_add(
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DependAddForm>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!(
        "insert into depend (course_id, user_id) value (?, ?)",
        form.course_id,
        form.user_id,
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
            resp_args = "failed add Depend";
        }
    }
    connection.detach();
    Json(Response::new(
        resp_str,
        ResponseData::String(resp_args.to_string()),
    ))
}

// 管理员删除课程学员
#[post("/Depend/delete", data = "<form>")]
pub async fn depend_delete_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    depend_delete(pool, form, None).await
}
// 用户删除课程学员
#[post("/Depend/delete", rank = 2, data = "<form>")]
pub async fn depend_delete_user(
    user: User,
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    depend_delete(pool, form, Some(user.id)).await
}
async fn depend_delete(
    pool: &rocket::State<Pool<MySql>>,
    form: Form<DeleteForm>,
    user_id: Option<i32>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let ok: bool;
    if let Some(id) = user_id {
        let row = sqlx::query!(
            "DELETE FROM depend WHERE id = ? AND user_id = ?",
            form.id,
            id
        )
        .execute(conn)
        .await;
        match row {
            Ok(_result) => ok = true,
            Err(_err) => ok = false,
        }
    } else {
        let row = sqlx::query!("DELETE FROM depend WHERE id = ?", form.id)
            .execute(conn)
            .await;
        match row {
            Ok(_result) => ok = true,
            Err(_err) => ok = false,
        }
    }
    let resp_str: &str;
    let resp_args: &str;
    if ok {
        resp_str = "success";
        resp_args = "some";
    } else {
        resp_str = "failed";
        resp_args = "Wrong while delete course";
    }
    connection.detach();
    Json(Response::new(
        resp_str,
        ResponseData::String(resp_args.to_string()),
    ))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Depend {
    id: i32,
    course_id: i32,
    user_id: i32,
}
// 管理员查询课程学员
#[post("/depend/query")]
pub async fn depend_query_admin(
    _admin: Admin,
    pool: &rocket::State<Pool<MySql>>,
) -> Json<Response> {
    depend_query(pool, None).await
}
// 用户查询课程学员
#[post("/depend/query", rank = 2)]
pub async fn depend_query_user(user: User, pool: &rocket::State<Pool<MySql>>) -> Json<Response> {
    depend_query(pool, Some(user.id)).await
}
async fn depend_query(pool: &rocket::State<Pool<MySql>>, id: Option<i32>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let response: Response;
    if let Some(user_id) = id {
        let row = sqlx::query_as!(Depend, "SELECT * FROM depend WHERE user_id = ?", user_id)
            .fetch_all(conn)
            .await;
        match row {
            Ok(result) => response = Response::new("success", ResponseData::Depend(result)),
            Err(_err) => {
                response = Response::new("Failed", ResponseData::String("Failed".to_string()));
            }
        }
    } else {
        let row = sqlx::query_as!(Depend, "SELECT * FROM depend")
            .fetch_all(conn)
            .await;
        match row {
            Ok(result) => response = Response::new("success", ResponseData::Depend(result)),
            Err(_err) => {
                response = Response::new("Failed", ResponseData::String("Failed".to_string()));
            }
        }
    }
    connection.detach();
    Json(response)
}
