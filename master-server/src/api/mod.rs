use crate::{db::with_db, env_config::ENV_CONFIG};
use sea_orm::DatabaseConnection;
use warp::{reject::Rejection, reply::Reply, Filter};

mod cards;
mod error;
pub mod types;

pub fn create_warp_route(
    db: DatabaseConnection,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let card_endpoint = warp::path!("api" / "campus_card" / String);

    let register_card_endpoint = card_endpoint
        .and(warp::put())
        .and(warp::header::exact("password", &ENV_CONFIG.password))
        .and(with_db(db.clone()))
        .then(cards::register_campus_card);

    let lookup_card_endpoint = card_endpoint
        .and(warp::get())
        .and(with_db(db.clone()))
        .then(cards::lookup_campus_card);

    let list_cards = warp::path!("api" / "campus_card")
        .and(with_db(db.clone()))
        .then(cards::list_campus_cards);

    let set_amonut = warp::path!("api" / "campus_card" / String / "set")
        .and(warp::post())
        .and(warp::header::exact("password", &ENV_CONFIG.password))
        .and(warp::query::<cards::SetParams>())
        .and(with_db(db.clone()))
        .then(cards::set_tape);

    let add_amount = warp::path!("api" / "campus_card" / String / "add")
        .and(warp::post())
        .and(warp::header::exact("password", &ENV_CONFIG.password))
        .and(warp::query::<cards::SetParams>())
        .and(with_db(db.clone()))
        .then(cards::add_tape);

    lookup_card_endpoint
        .or(register_card_endpoint)
        .or(list_cards)
        .or(set_amonut)
        .or(add_amount)
}
