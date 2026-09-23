use tokio::signal::ctrl_c;

pub async fn shutdown_signal() {
    let _ = ctrl_c().await;
}
