use crate::utils::{Admin, Coach, Course, DeleteForm, Response, ResponseData, User};
use rocket::{form::Form, serde::json::Json, State};
use sqlx::{MySql, Pool};

//这里还要再改
#[derive(FromForm)]
pub struct CourseAddForm<'r> {
    course_name: &'r str,
    course_describe: &'r str,
    coach_id: i32,
}
//管理员添加课程
#[post("/course/add", data = "<form>")]
pub async fn course_add_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<CourseAddForm<'_>>,
) -> Json<Response> {
    course_add(form, pool).await
}
//教练添加课程
#[post("/course/add", rank = 2, data = "<form>")]
pub async fn course_add_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<CourseAddForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.coach_id = coach.get_id();
    course_add(changed_form, pool).await
}

async fn course_add(form: Form<CourseAddForm<'_>>, pool: &State<Pool<MySql>>) -> Json<Response> {
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

//管理员删除课程
#[post("/course/delete", data = "<form>")]
pub async fn course_delete_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    course_delete(pool, form, None).await
}
//教练删除课程
#[post("/course/delete", rank = 2, data = "<form>")]
pub async fn course_delete_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<DeleteForm>,
) -> Json<Response> {
    course_delete(pool, form, Some(coach.get_id())).await
}
async fn course_delete(
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

#[get("/course/query")]
pub async fn course_query(_user: User, pool: &State<Pool<MySql>>) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query_as!(Course, "SELECT * FROM course")
        .fetch_all(conn)
        .await;
    let response: Response;
    match row {
        Ok(result) => response = Response::new("success", ResponseData::Courses(result)),
        Err(_err) => {
            response = Response::new("Failed", ResponseData::String("Failed".to_string()));
        }
    }
    connection.detach();
    Json(response)
}
#[get("/course/query/<id>")]
pub async fn course_query_where_coach(_coach: Coach, pool: &State<Pool<MySql>>, id: i32) -> Json<Response> {
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");
    let conn = connection.as_mut();
    let row = sqlx::query_as!(Course, "SELECT * FROM course WHERE coach_id = ?", id)
        .fetch_all(conn)
        .await;
    let response: Response;
    match row {
        Ok(result) => response = Response::new("success", ResponseData::Courses(result)),
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
    course_describe: &'r str,
    coach_id: i32,
}
//管理员更改课程
#[post("/course/change", data = "<form>")]
pub async fn course_change_admin(
    _admin: Admin,
    pool: &State<Pool<MySql>>,
    form: Form<CourseChangeForm<'_>>,
) -> Json<Response> {
    course_change(pool, form).await
}
//教练更改课程
#[post("/course/change", rank = 2, data = "<form>")]
pub async fn course_change_coach(
    coach: Coach,
    pool: &State<Pool<MySql>>,
    form: Form<CourseChangeForm<'_>>,
) -> Json<Response> {
    let mut changed_form = form;
    changed_form.coach_id = coach.get_id();
    course_change(pool, changed_form).await
}
async fn course_change(
    pool: &State<Pool<MySql>>,
    form: Form<CourseChangeForm<'_>>,
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
