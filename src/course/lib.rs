use crate::utils::{Course, DeleteForm, Response, ResponseData};
use rocket::{form::Form, serde::json::Json, State};
use sqlx::{MySql, Pool};

#[derive(FromForm)]
pub struct AddForm<'r> {
    course_name: &'r str,
    course_describe: &'r str,
    pub coach_id: i32,
}
#[derive(FromForm)]
pub struct ChangeForm<'r> {
    id: i32,
    course_name: &'r str,
    course_describe: &'r str,
    pub coach_id: i32,
}

pub async fn add(
    pool: &State<Pool<MySql>>,
    form: Form<AddForm<'_>>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let res = sqlx::query!(
        "select * from course where course_name = ? AND coach_id = ?",
        form.course_name,
        form.coach_id
    )
    .fetch_one(conn)
    .await;
    let resp_str: &str;
    let resp_args: &str;
    match res {
        Ok(_result) => {
            resp_str = "failed";
            resp_args = "course already exist";
        }
        Err(_err) => {
            let conn = connection.as_mut();
            let row = sqlx::query!(
                "insert into course (course_name, course_describe, coach_id) value (?, ?, ?)",
                form.course_name,
                form.course_describe,
                form.coach_id,
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
                    resp_args = "Wrong while add course";
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

pub async fn delete(
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
    coach_id: Option<i32>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let ok: bool;
    if let Some(id) = coach_id {
        let row = sqlx::query!(
            "DELETE FROM course WHERE id = ? AND coach_id = ?",
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
        let row = sqlx::query!("DELETE FROM course WHERE id = ?", form.id)
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

pub async fn change(
    pool: &State<Pool<MySql>>,
    form: Form<ChangeForm<'_>>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!(
        "UPDATE course
        SET course_name = ?, course_describe = ?, coach_id = ?
        WHERE id = ?",
        form.course_name,
        form.course_describe,
        form.coach_id,
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

pub async fn query(pool: &State<Pool<MySql>>, coach_id: Option<i32>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let response: Response;
    if let Some(id) = coach_id {
        let row = sqlx::query_as!(Course, "SELECT * FROM course WHERE coach_id = ?", id)
            .fetch_all(conn)
            .await;
        match row {
            Ok(result) => response = Response::new("success", ResponseData::Courses(result)),
            Err(_err) => {
                response = Response::new("Failed", ResponseData::String("Failed".to_string()));
            }
        }
    } else {
        let row = sqlx::query_as!(Course, "SELECT * FROM course")
            .fetch_all(conn)
            .await;
        match row {
            Ok(result) => response = Response::new("success", ResponseData::Courses(result)),
            Err(_err) => {
                response = Response::new("Failed", ResponseData::String("Failed".to_string()));
            }
        }
    }
    connection.detach();
    Json(response)
}
