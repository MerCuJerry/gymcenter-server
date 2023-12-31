use rocket::{
    http::Status,
    outcome::try_outcome,
    request::{FromRequest, Outcome},
    serde::{Serialize, ser::{SerializeStruct, Serializer}},
    Request,
};
use base64ct::{Base64, Encoding};
use sqlx::{MySql, Pool};
use sha2::{Sha256, Digest};

// 可能返回的数据
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ResponseData {
    User(User),
    Users(Vec<User>),
    Notice(Notice),
    Courses(Vec<Course>),
    String(String),
}
// 返回数据
pub struct Response {
    result: String,
    data: ResponseData,
}
impl Response {
    pub fn new(result: &str, data: ResponseData) -> Response {
        Response {
            result: String::from(result),
            data,
        }
    }
}
impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("No", 2)?;
        s.serialize_field("result", &self.result)?;
        match &self.data {
            ResponseData::User(arg) =>  s.serialize_field("data", arg)?,
            ResponseData::Users(arg) =>  s.serialize_field("data", arg)?,
            ResponseData::Notice(arg) =>  s.serialize_field("data", arg)?,
            ResponseData::Courses(arg) =>  s.serialize_field("data", arg)?,
            ResponseData::String(arg) =>  s.serialize_field("data", arg)?,
        }
        s.end()
    }
}

#[derive(FromForm)]
pub struct DeleteForm {
    pub id: i32,
}

//检查用户权限
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub password: String,
    pub phone_num: String,
    pub permission: String,
    pub self_sign: Option<String>,
}

pub struct Coach(User);
impl Coach {
    pub fn get_id(&self) -> i32 {
        self.0.id
    }
}
pub struct Admin(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<User, ()> {
        let pool = request
            .rocket()
            .state::<Pool<MySql>>()
            .expect("Falied to get connection");
        if let (Some(id), Some(password)) = (
            request.cookies().get_private("user_id"),
            request.cookies().get_private("token"),
        ) {
            let user_id = id.value_trimmed();
            let user_password = Base64::encode_string(
                Sha256::new()
                    .chain_update(password.value_trimmed())
                    .finalize()
                    .as_ref(),
            );
            let mut connection = pool.acquire().await.expect("Failed to acquire connection");
            let conn = connection.as_mut();
            let res = sqlx::query_as!(
                User,
                "SELECT * FROM user WHERE id = ? AND password = ?",
                user_id,
                user_password
            )
            .fetch_one(conn)
            .await;
            connection.detach();
            match res {
                Ok(result) => Outcome::Success(result),
                Err(_err) => Outcome::Forward(Status::Unauthorized),
            }
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Coach {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Coach, ()> {
        let user = try_outcome!(request.guard::<User>().await);
        if user.permission != "user" {
            Outcome::Success(Coach(user))
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Admin, ()> {
        let user = try_outcome!(request.guard::<User>().await);
        if user.permission == "admin" {
            Outcome::Success(Admin(user))
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Course {
    pub id: i32,
    pub course_name: String,
    pub course_describe: String,
    pub coach_id: i32,
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notice {
    pub id: i32,
    pub notice_content: String,
}