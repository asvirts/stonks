use warp::Filter;
use serde::{Deserialize, Serialize};
use reqwest::Error;

// Define a struct to represent the JSON response
#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    // Adjust fields based on the actual API response
    field1: String,
    field2: String,
}

#[tokio::main]
async fn main() {
    // Define a route that makes an API call
    let api_route = warp::path("api")
        .and(warp::get())
        .and_then(handle_api_request);

    // Start the server on port 3030
    warp::serve(api_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_api_request() -> Result<impl warp::Reply, warp::Rejection> {
    match make_api_call().await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn make_api_call() -> Result<ApiResponse, Error> {
    // Replace with the actual URL of the API you want to call
    let url: &str = "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&apikey=demo";
    
    // Make the API call
    let response = reqwest::get(url).await?;
    
    // Parse the JSON response
    let api_response = response.json::<ApiResponse>().await?;
    
    Ok(api_response)
}
