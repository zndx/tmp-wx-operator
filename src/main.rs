use futures::{StreamExt, TryStreamExt};
use tracing::{event, Level};
use k8s_openapi::{api::core::v1::Pod};
use kube::{Api, api::{ListParams, ObjectMeta, PostParams, WatchEvent}, client::Client, Error, Resource};


static NAMESPACE: &str = "kube-system";

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        // sets this to be the default, global collector for this application.
        .init();
    let client = Client::try_default().await.unwrap();
    let api: Api<Pod> = Api::namespaced(client, NAMESPACE);
    let mut stream = api.watch(&ListParams::default(), "0").await?.boxed();
    while let Some(event) = stream.try_next().await? {
        match event {
            WatchEvent::Added(pod) => event!(Level::INFO, "ADDED: {}", pod.name()),
            WatchEvent::Modified(pod) => event!(Level::INFO, "UPDATED: {}", pod.name()),
            WatchEvent::Deleted(pod) => event!(Level::INFO, "DELETED: {}", pod.name()),
            WatchEvent::Error(e) => event!(Level::ERROR, "ERROR: {} {} ({})", e.code, e.message, e.status),
            _ => {}
        };
    }
    event!(Level::INFO, "Controller finished successfully");
    Ok(())
}

