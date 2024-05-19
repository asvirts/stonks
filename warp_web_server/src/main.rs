use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a basic route
    let hello = warp::path::end()
        .map(|| warp::reply::html("Hi, mom!"));

    // Start the server on port 3030
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
