mod config;
use config::{read_config, Configuration};
use serde::Serialize;
mod models;
mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
use crate::diesel::sql_types;
use crate::diesel::Insertable;
use crate::diesel::IntoSql;
use crate::diesel::RunQueryDsl;
use crate::diesel::{QueryDsl, QueryResult};
use chrono::NaiveDateTime;
use diesel::ExpressionMethods;
use diesel::{pg, r2d2};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket::serde::json::Json;
use schema::habitmetric;
use schema::habitname;

embed_migrations!("./migrations");

struct AuthorizationGuard {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizationGuard {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let config: &Configuration = req.rocket().state().unwrap();
        let mut auth_iter = req.headers().get("Authorization");
        let value = auth_iter.next();
        if value.is_none() {
            return Outcome::Failure((Status::Unauthorized, "Not authenticated".to_string()));
        }
        if !config.auth_whitelist.contains(&value.unwrap().to_string()) {
            return Outcome::Failure((Status::Unauthorized, "Not authenticated".to_string()));
        }
        return Outcome::Success(AuthorizationGuard {});
    }
}

struct PostgresCon {
    pub con: r2d2::PooledConnection<r2d2::ConnectionManager<diesel::PgConnection>>,
}

impl PostgresCon {
    pub fn get(self) -> r2d2::PooledConnection<r2d2::ConnectionManager<diesel::PgConnection>> {
        self.con
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PostgresCon {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool: &r2d2::Pool<r2d2::ConnectionManager<pg::PgConnection>> =
            req.rocket().state().unwrap();
        let con = pool.get();
        if con.is_err() {
            return Outcome::Failure((
                Status::InternalServerError,
                "DB Connection Error\n".to_string(),
            ));
        }
        return Outcome::Success(PostgresCon { con: con.unwrap() });
    }
}

#[post("/habit/create?<name>")]
fn habit_create(
    _authorized: AuthorizationGuard,
    name: String,
    con: PostgresCon,
) -> status::Custom<String> {
    let con = con.get();

    let new_habit = models::NewHabit {
        name: name.as_str(),
    };
    let result: Result<i32, _> = new_habit
        .insert_into(habitname::dsl::habitname)
        .returning(habitname::dsl::id)
        .get_result(&con);

    if result.is_err() {
        return status::Custom(
            Status::BadRequest,
            "Insert error (already inserted?)\n".to_string(),
        );
    }

    status::Custom(Status::Created, format!("{}\n", result.unwrap()))
}

#[get("/habit/list")]
fn habit_list(
    _authorized: AuthorizationGuard,
    con: PostgresCon,
) -> status::Custom<Result<Json<Vec<String>>, String>> {
    let con = con.get();
    let res: QueryResult<Vec<String>> = habitname::dsl::habitname
        .select(habitname::dsl::name)
        .load(&con);
    if res.is_err() {
        return status::Custom(
            Status::InternalServerError,
            Err("DB Query Error\n".to_string()),
        );
    }
    status::Custom(Status::Ok, Ok(Json(res.unwrap())))
}

#[post("/habitentry/create?<name>&<units>")]
fn habitentry_create(
    _authorized: AuthorizationGuard,
    name: String,
    units: i32,
    con: PostgresCon,
) -> status::Custom<String> {
    let con = con.get();

    let result: Result<i32, _> = habitname::table
        .select((habitname::dsl::id, units.into_sql::<sql_types::Integer>()))
        .filter(habitname::dsl::name.eq(name))
        .insert_into(habitmetric::table)
        .into_columns((habitmetric::habitname_id, habitmetric::units))
        .returning(habitmetric::dsl::id)
        .get_result(&con);

    if result.is_err() {
        println!("Insert error: {:?}", result.err());
        return status::Custom(Status::BadRequest, "Insert error\n".to_string());
    }

    status::Custom(Status::Created, format!("{}\n", result.unwrap()))
}

#[derive(Serialize)]
pub struct HabitEntry {
    id: u32,
    timestamp: i64,
    units: i32,
    habit_name: String,
}

#[get("/habitentry/list?<name>&<limit>")]
fn habitentry_list(
    _authorized: AuthorizationGuard,
    name: Option<String>,
    limit: Option<i64>,
    con: PostgresCon,
) -> status::Custom<Result<Json<Vec<HabitEntry>>, String>> {
    let con = con.get();
    let builder1 = habitmetric::dsl::habitmetric
        .inner_join(habitname::dsl::habitname)
        .select((
            habitmetric::dsl::id,
            habitmetric::dsl::datetime,
            habitmetric::dsl::units,
            habitname::dsl::name,
        ));
    let res: QueryResult<Vec<(i32, NaiveDateTime, i32, String)>> = if name.is_some() {
        builder1
            .filter(habitname::dsl::name.eq(name.unwrap()))
            .limit(limit.unwrap_or(1000))
            .load(&con)
    } else {
        builder1.limit(limit.unwrap_or(1000)).load(&con)
    };

    if res.is_err() {
        return status::Custom(
            Status::InternalServerError,
            Err("DB Query Error\n".to_string()),
        );
    }

    let result = res
        .unwrap()
        .iter()
        .map(|x| HabitEntry {
            id: x.0 as u32,
            timestamp: x.1.timestamp(),
            units: x.2,
            habit_name: x.3.clone(),
        })
        .collect::<Vec<HabitEntry>>();

    status::Custom(Status::Ok, Ok(Json(result)))
}

#[post("/habitentry/delete?<id>")]
fn habitentry_delete(
    _authorized: AuthorizationGuard,
    id: i32,
    con: PostgresCon,
) -> status::Custom<Result<String, String>> {
    let con = con.get();
    let res =
        diesel::delete(habitmetric::dsl::habitmetric.filter(habitmetric::id.eq(id))).execute(&con);
    if res.is_err() {
        return status::Custom(
            Status::InternalServerError,
            Err("DB Query Error\n".to_string()),
        );
    }

    status::Custom(Status::Ok, Ok(String::new()))
}

#[launch]
fn rocket() -> _ {
    let config = read_config();
    let manager = r2d2::ConnectionManager::<pg::PgConnection>::new(config.postgres.clone());
    let db_pool = r2d2::Builder::new()
        .max_size(32)
        .min_idle(Some(4))
        .build(manager)
        .unwrap();

    embedded_migrations::run(
        &db_pool
            .get()
            .expect("Could not get initial DB connection to run migrations"),
    )
    .expect("Failed to run migrations");
    println!("Database is up to date!");

    rocket::build().manage(db_pool).manage(config).mount(
        "/",
        routes![
            habit_create,
            habit_list,
            habitentry_create,
            habitentry_list,
            habitentry_delete
        ],
    )
}
