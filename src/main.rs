use reqwest::Error;
mod containers;
use containers::Containers;

async fn fetch_containers() -> Result<Containers, Error> {
    let url = "http://localhost:2375/containers/json";
    let containers = reqwest::get(url)
    .await?
    .json::<Containers>()
    .await?;
    Ok(containers)
}

#[tokio::main]
async fn main() {
    match fetch_containers().await {
        Ok(containers) => {
            for container in containers {
                println!("{:#?}", container.basic_list());
            }
        },
        Err(e) => eprintln!("Failed to fetch containers: {}", e),
    }
}
