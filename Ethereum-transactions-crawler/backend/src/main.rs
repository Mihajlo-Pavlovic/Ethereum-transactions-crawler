use common::models as c_models;
use reqwest::Client;
use std::convert::Infallible;
use warp::{Filter, Rejection};

mod client;
mod etherscan;
mod handlers;

const API_URL: &str = "https://api.etherscan.io/api";

type WarpResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    dotenv::dotenv().ok();
    log4rs::init_file("logging_config.yml", Default::default()).unwrap();

    let http_client = client::get_client();

    let root = warp::path::end().map(|| "Welcome to my api");
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Access-Control-Allow-Origin",
            "Origin",
            "Accept",
            "X-Requested-With",
            "Content-Type",
        ])
        .allow_methods(vec!["GET", "POST"]);

    let account_route = warp::path("account")
        .and(warp::get())
        .and(with_http_client(http_client.clone()))
        .and(warp::query::<c_models::QueryParams>())
        .and_then(handlers::get_account_data)
        .with(&cors);
    let routes = root.or(account_route).with(&cors);

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}

fn with_http_client(
    client: Client,
) -> impl Filter<Extract = (Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}
