use crate::models::container::Containers;
use reqwest::blocking;
use reqwest::Error;

pub fn stop_container(container_id: &String) -> Result<(), Error> {
    let client = blocking::Client::new();
    let url = format!("http://localhost:2375/containers/{}/stop", container_id);

    let res = client.post(url).send()?;
    if res.status().is_success() {
        println!("Container stopped successfully.");
    } else {
        println!("Failed to stop container: Status {}", res.status());
    }
    Ok(())
}

pub fn fetch_containers() -> Result<Containers, Error> {
    let url = "http://localhost:2375/containers/json";
    let response = blocking::get(url)?;
    let containers = response.json::<Containers>()?;
    Ok(containers)
}
