use crate::{db::with_db, env_config::ENV_CONFIG};
use sea_orm::DatabaseConnection;
use utoipa_redoc::Redoc;
use warp::{reject::Rejection, reply::Reply, Filter};

mod cards;
pub mod error;
pub mod types;

fn auth() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::exact("auth", &ENV_CONFIG.password)
}

#[cfg(not(debug_assertions))]
async fn api_docs() -> impl warp::Reply {
    use warp::http::{header, Response};

    Response::builder()
        .header(header::CONTENT_TYPE, "text/x-yaml")
        .body(include_str!("../../open-api.yaml"))
}

#[cfg(debug_assertions)]
async fn api_docs() -> impl warp::Reply {
    use warp::http::{header, Response};

    Response::builder()
        .header(header::CONTENT_TYPE, "text/x-yaml")
        .body(std::fs::read_to_string("open-api.yaml").expect("Could not read openapi docs"))
}

pub fn create_warp_route(
    db: DatabaseConnection,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let openapi = warp::path!("openapi.yaml").and(warp::get()).then(api_docs);
    let redoc_ui = Redoc::new("/openapi.yaml");
    let redoc = warp::path("docs")
        .and(warp::get())
        .map(move || warp::reply::html(redoc_ui.to_html()));

    let login = warp::path!("login")
        .and(warp::get())
        .and(auth())
        .then(|| async { "" });

    let list_cards = warp::path!("api" / "campus_card")
        .and(with_db(db.clone()))
        .then(cards::list_campus_cards);

    let add_all = warp::path!("api" / "campus_card" / "add")
        .and(warp::post())
        .and(auth())
        .and(warp::query::<cards::SetParams>())
        .and(with_db(db.clone()))
        .then(cards::add_all);

    let set_all = warp::path!("api" / "campus_card" / "set")
        .and(warp::post())
        .and(auth())
        .and(warp::query::<cards::SetParams>())
        .and(with_db(db.clone()))
        .then(cards::set_all);

    let register_card_endpoint = warp::path!("api" / "campus_card" / String)
        .and(warp::put())
        .and(auth())
        .and(with_db(db.clone()))
        .then(cards::register_campus_card);

    let lookup_card_endpoint = warp::path!("api" / "campus_card" / String)
        .and(warp::get())
        .and(with_db(db.clone()))
        .then(cards::lookup_campus_card);

    let set_amonut = warp::path!("api" / "campus_card" / String / "set")
        .and(warp::post())
        .and(auth())
        .and(warp::query::<cards::SetParams>())
        .and(with_db(db.clone()))
        .then(cards::set_tape);

    let add_amount = warp::path!("api" / "campus_card" / String / "add")
        .and(warp::post())
        .and(auth())
        .and(warp::query::<cards::SetParams>())
        .and(with_db(db.clone()))
        .then(cards::add_tape);

    lookup_card_endpoint
        .or(login)
        .or(add_all)
        .or(set_all)
        .or(register_card_endpoint)
        .or(list_cards)
        .or(set_amonut)
        .or(add_amount)
        .or(openapi)
        .or(redoc)
}
