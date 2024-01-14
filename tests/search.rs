use tvdb4::apis::search_api;

mod setup;

#[tokio::test]
async fn test_search() {
    let res = search_api::get_search_results(
        &setup::setup().await,
        Some("月が導く異世界道中 第二幕"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    println!("{res:?}");
}
