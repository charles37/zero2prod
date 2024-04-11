use requwest;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn our app.");

}
