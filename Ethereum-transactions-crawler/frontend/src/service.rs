use common::models as c_models;
use gloo_net::http::Request;

const API_URL: &str = "http://localhost:5000";

pub async fn get_account_data(
    params: &common::models::QueryParams,
) -> Option<c_models::AccountData> {
    let request_url = format!(
        "{}/account?address={}&page={}&offset={}&sort={}&from={}&to={}",
        API_URL,
        &params.address,
        params.page,
        params.offset,
        &params.sort,
        &params.from,
        &params.to
    );

    log::debug!("request: {}", &request_url);
    let response = Request::get(&request_url).send().await.unwrap();

    let result = if response.ok() {
        let body = response.text().await.unwrap();

        log::debug!("Response: {}", &body);

        let obj = serde_json::from_str::<c_models::AccountData>(&body).unwrap();
        Some(obj)
    } else {
        None
    };

    result
}
