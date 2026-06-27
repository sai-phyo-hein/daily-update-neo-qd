use neo4rs::{query, Graph, ConfigBuilder};
use std::env;

pub async fn ping(now: String) -> Result<(), Box<dyn std::error::Error>> {

    // dotenvy::dotenv().ok();
    let uri = env::var("NEO4J_URI").expect("NEO4J_URI not set");
    let user = env::var("NEO4J_USER").expect("NEO4J_USER not set");
    let pass = env::var("NEO4J_PASS").expect("NEO4J_PASS not set");
    let db   = env::var("NEO4J_DATABASE").unwrap_or_else(|_| "neo4j".to_string());

    println!("Connecting to Neo4j Aura...");
    let config = ConfigBuilder::default()
        .uri(&uri)
        .user(&user)
        .password(&pass)
        .db(&*db)
        .build()?;

    let graph = Graph::connect(config).await?;

    graph
        .run(
            query(
                "MERGE (k:DummyNode {id: 'dummykeepalive'})
                SET k.last_ping = $time,
                    k.ping_count = coalesce(k.ping_count, 0) + 1
                "
            )
            .param("time", now.clone())
        )
        .await?;

    println!("✅ Neo4j - last_ping = {now}");
    Ok(())
}