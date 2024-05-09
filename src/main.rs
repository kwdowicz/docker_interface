use reqwest::Error;
use warp::Filter;
mod containers;
use containers::Containers;

async fn containers() -> Result<Containers, Error> {
    let url = "http://localhost:2375/containers/json";
    let containers = reqwest::get(url)
    .await?
    .json::<Containers>()
    .await?;
    Ok(containers)
}

async fn handle_containers() -> Result<impl warp::Reply, warp::Rejection> {
    match containers().await {
        Ok(containers) => Ok(warp::reply::json(&containers)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() {
    // Define a route at `/containers` that executes handle_containers
    let containers_route = warp::path("containers")
        .and(warp::get())
        .and_then(handle_containers);

    // Start the warp server on port 3030
    warp::serve(containers_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
