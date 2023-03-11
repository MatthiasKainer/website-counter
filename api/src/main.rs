extern crate dotenv;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::{env, fmt};

use anyhow::Result;

use rocket::http::{Status, Header};
use rocket::{State, Request, Response};
use rocket::response::content;
use rocket::fairing::{Fairing, Info, Kind};

use std::time::SystemTime;

use sqlx::{PgPool, Pool, Postgres};

#[derive(Debug)]
pub struct Counter {
    pub page_views: i64,
    pub visitors: i64,
    pub sessions: i64,
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \"page_views\": {}, \"visitors\": {}, \"sessions\": {} }}", self.page_views, self.visitors, self.sessions)
    }
}

impl Counter {
    pub async fn track_page_view(
        url: String,
        visitor: String,
        session: String,
        pool: &Pool<Postgres>,
    ) -> Result<String> {
        let result = sqlx::query!("INSERT INTO counter_log (url, userid, sessionid, timestamp) VALUES ($1, $2, $3, $4) RETURNING url", url, visitor, session, SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis().to_string())
            .fetch_one(pool)
            .await?;
        Ok(result.url)
    }

    pub async fn find_by_url(url: String, pool: &Pool<Postgres>) -> Result<Counter> {
        let row: (i64,i64,i64) = sqlx::query_as("SELECT
        COUNT (*) AS url_count,
        COUNT (DISTINCT userid) AS userid_count,
        COUNT (DISTINCT sessionid) AS sessionid_count
      FROM counter_log WHERE url = $1")
            .bind(url)
            .fetch_one(pool)
            .await?;

        let counter = Counter {
            page_views: row.0,
            visitors: row.1,
            sessions: row.2
        };
        
        Ok(counter)
    }
}

#[get("/count/<url>/<visitor>/<session>")]
async fn count(
    pool: &State<Pool<Postgres>>,
    url: String,
    visitor: String,
    session: String,
) -> Result<content::RawJson<String>, Status> {
    // deal with this by logging and swallowing it
    let result = Counter::track_page_view(url.clone(), visitor, session, pool).await;
    if result.is_err() {
        error!("Could not track page view: {}", result.err().unwrap());
    }

    let count = Counter::find_by_url(url, &pool).await;

    match count {
        Ok(count) => Ok(content::RawJson(format!("{}", count))),
        _ => Err(Status::NotFound),
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    info!("Env set up. Database url: {}", database_url);

    let pool = PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let _rocket = rocket::build()
        .attach(CORS)
        .mount("/", routes![count])
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}
