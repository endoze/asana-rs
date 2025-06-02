#![allow(unused_imports)]
use asana::model::*;
use asana::{default_http_client, AsanaClient};

#[tokio::main]
async fn main() {
    let custom_client = default_http_client().base_url("https://app.asana.com/api/1.0");

    let client = AsanaClient::with_client_from_env(custom_client);
    let task_gid = "your task gid";
    let response = client
        .get_task(task_gid)
        .opt_fields(["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}

