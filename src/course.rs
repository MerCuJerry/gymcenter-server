use crate::utils::{Response, ResponseData, DeleteForm};
use rocket::{
    form::Form,
    serde::{json::Json, Serialize},
};
use sqlx::{MySql, Pool};

#[derive(FromForm)]
pub struct CourseAddForm<'r> {
    course_name: &'r str,
    course_discribe: &'r str,
    coach_id: i32,
}
#[post("/course/add", data = "<form>")]
pub async fn course_add(
    pool: &rocket::State<Pool<MySql>>,
    form: Form<CourseAddForm<'_>>,
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
            resp_args = "account already exist";
        }
        Err(_err) => {
            let conn = connection.as_mut();
            let row = sqlx::query!(
                "insert into course (course_name, course_discribe, coach_id) value (?, ?, ?)",
                form.course_name,
                form.course_discribe,
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

#[post("/course/delete", data = "<form>")]
pub async fn course_delete(pool: &rocket::State<Pool<MySql>>, form: Form<DeleteForm>,) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!("DELETE FROM course WHERE id = ?", form.id)
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
            resp_args = "Wrong while delete course";
        }
    }
    connection.detach();
    Json(Response::new(
        resp_str,
        ResponseData::String(resp_args.to_string()),
    ))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Course {
    id: i32,
    course_name: String,
    course_discribe: String,
    coach_id: i32,
}
#[get("/course/query")]
pub async fn course_query(pool: &rocket::State<Pool<MySql>>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!("SELECT * FROM course").fetch_all(conn).await;
    let response: Response;
    let course_vec: Vec<Course>;
    match row {
        Ok(_result) => {
            course_vec = _result
                .iter()
                .map(|x| Course {
                    id: x.id,
                    course_name: x.course_name.clone(),
                    course_discribe: x.course_discribe.clone(),
                    coach_id: x.coach_id.clone(),
                })
                .collect();
            response = Response::new("success", ResponseData::Course(course_vec))
        }
        Err(_err) => {
            response = Response::new("Failed", ResponseData::String("Failed".to_string()));
        }
    }
    connection.detach();
    Json(response)
}

#[derive(FromForm)]
pub struct CourseChangeForm<'r> {
    id: i32,
    course_name: &'r str,
    course_discribe: &'r str,
    coach_id: i32,
}
#[post("/course/change", data = "<form>")]
pub async fn course_change(pool: &rocket::State<Pool<MySql>>, form: Form<CourseChangeForm<'_>>,) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query!(
        "UPDATE course
        SET course_name = ?, course_discribe = ?, coach_id = ?
        WHERE id = ?",
        form.course_name,
        form.course_discribe,
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
