mod neo4j;
mod qdrant;

use chrono::Utc;

#[tokio::main]
async fn main() {
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let (neo4j_result, qdrant_result) = tokio::join!(
        neo4j::ping(now.clone()),
        qdrant::ping(now.clone())
    );

    if let Err(e) = neo4j_result {
        eprintln!("❌ Neo4j error: {e}");
        std::process::exit(1);
    }
    if let Err(e) = qdrant_result {
        eprintln!("❌ Qdrant error: {e}");
        std::process::exit(1);
    }
}