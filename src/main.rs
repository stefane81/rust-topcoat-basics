mod app;

#[tokio::main]
async fn main() {
    // topcoat::start(
    //     Router::builder()
    //         .layout(app::root_layout)
    //         .discover()
    //         .assets(AssetBundle::load_dir("target/assets").unwrap())
    //         .build(),
    // )
    // .await
    // .unwrap();

    topcoat::start(app::router()).await.unwrap();
}
