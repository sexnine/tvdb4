use tvdb4::{apis::configuration::Configuration, models::LoginPostRequest};

pub async fn setup() -> Configuration {
    let api_key = std::env::var("TVDB_API_KEY").unwrap();

    let mut cfg = Configuration::new();
    let token = tvdb4::apis::login_api::login_post(&cfg, LoginPostRequest::new(api_key))
        .await
        .unwrap()
        .data
        .unwrap()
        .token;
    cfg.bearer_access_token = token;
    cfg
}
