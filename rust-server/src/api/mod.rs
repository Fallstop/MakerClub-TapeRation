use sea_orm::DatabaseConnection;
use warp::{ reject::Rejection, reply::Reply, Filter};

use crate::db::with_db;
mod cards;
mod error;

pub fn create_warp_route(db: DatabaseConnection) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let card_endpoint = warp::path!("api" / "campus_card" / String);
    
    let register_card_endpoint = card_endpoint
        .and(warp::post())
        .and(with_db(db.clone()))
        .and_then(cards::register_campus_card);

    let lookup_card_endpoint = card_endpoint
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(cards::lookup_campus_card);

    let list_cards = warp::path!("api" / "campus_card")
        .and(with_db(db.clone()))
        .and_then(cards::list_campus_cards);
    

    return lookup_card_endpoint.or(register_card_endpoint).or(list_cards);
}
