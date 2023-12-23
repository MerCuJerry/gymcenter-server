use crate::utils::{User, Course, DeleteForm, Response, ResponseData};
use rocket::{State, form::Form, serde::json::Json};
use sqlx::{MySql, Pool};

#[derive(FromForm)]
pub struct AddForm {
    course_id: i32,
    pub user_id: i32,
}
pub enum QueryType {
    Course,
    Coach,
    User,
}
pub async fn add(
    pool: &State<Pool<MySql>>,
    form: Form<AddForm>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let check = sqlx::query!(
        "SELECT * FROM depend WHERE course_id = ? AND user_id = ?",
        form.course_id,
        form.user_id,
    )
    .fetch_one(connection.as_mut())
    .await;
    let resp_str: &str;
    let resp_args: &str;
    match check {
        Ok(_result) => {
            resp_str = "failed";
            resp_args = "failed add Depend";
        }
        Err(_err) => {
            let row = sqlx::query!(
                "insert into depend (course_id, user_id) value (?, ?)",
                form.course_id,
                form.user_id,
            )
            .execute(connection.as_mut())
            .await;
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
    user_id: Option<i32>,
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let ok: bool;
    if let Some(id) = user_id {
        let row = sqlx::query!(
            "DELETE FROM depend WHERE course_id = ? AND user_id = ?",
            form.id,
            id
        )
        .execute(connection.as_mut())
        .await;
        match row {
            Ok(_result) => ok = true,
            Err(_err) => ok = false,
        }
    } else {
        let row = sqlx::query!("DELETE FROM depend WHERE id = ?", form.id)
            .execute(connection.as_mut())
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

pub async fn query(
    pool: &State<Pool<MySql>>,
    query_type: QueryType,
    id: i32
) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let response: Response;
    match query_type {
        QueryType::Course => {
            let row = sqlx::query_as!(
                Course,
                "SELECT * FROM course WHERE id IN (
                    SELECT course_id FROM depend where user_id = ?
                )",
                id
            )
            .fetch_all(connection.as_mut())
            .await;
            match row {
                Ok(result) => response = Response::new("success", ResponseData::Courses(result)),
                Err(_err) => {
                    response = Response::new("Failed", ResponseData::String("Failed".to_string()));
                }
            }
        },
        QueryType::Coach => {
            let row = sqlx::query_as!(
                User,
                "SELECT * FROM user WHERE id IN (
                    SELECT coach_id FROM course WHERE id IN (
                        SELECT course_id FROM depend where user_id = ?
                    )
                )",
                id
            )
            .fetch_all(connection.as_mut())
            .await;
            match row {
                Ok(result) => response = Response::new("success", ResponseData::Users(result)),
                Err(_err) => {
                    response = Response::new("Failed", ResponseData::String("Failed".to_string()));
                }
            }
        },
        QueryType::User => {
            let row = sqlx::query_as!(
                User,
                "SELECT * FROM user WHERE id IN (
                    SELECT user_id FROM depend WHERE course_id = ?
                )",
                id
            )
            .fetch_all(connection.as_mut())
            .await;
            match row {
                Ok(result) => response = Response::new("success", ResponseData::Users(result)),
                Err(_err) => {
                    response = Response::new("Failed", ResponseData::String("Failed".to_string()));
                }
            }
        }
    }
    connection.detach();
    Json(response)
}