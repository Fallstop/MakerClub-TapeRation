use crate::{db::with_db, env_config::ENV_CONFIG};
use sea_orm::DatabaseConnection;
use warp::{reject::Rejection, reply::Reply, Filter};

mod cards;
mod error;
pub mod types;

fn auth() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::exact("password", &ENV_CONFIG.password)
}

pub fn create_warp_route(
    db: DatabaseConnection,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
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
}
