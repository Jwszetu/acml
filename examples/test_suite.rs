use anyhow::Result;
use serde_json::json;


#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;

    hc.do_get("/test").await?.print().await?;

    Ok(())
}