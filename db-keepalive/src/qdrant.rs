use qdrant_client::{
    qdrant::{
        value::Kind, CreateCollectionBuilder, Distance, PointStruct,
        UpsertPointsBuilder, Value, VectorParamsBuilder,
    },
    Qdrant,
};
use std::{collections::HashMap, env};

const COLLECTION: &str = "dummykeepalive";
const POINT_ID: u64 = 1;

pub async fn ping(now: String) -> Result<(), Box<dyn std::error::Error>> {

    // dotenvy::dotenv().ok();
    let url = env::var("QDRANT_URL").expect("QDRANT_URL not set");
    let api_key = env::var("QDRANT_API_KEY").expect("QDRANT_API_KEY not set");

    println!("Connecting to Qdrant Cloud ...");
    let client = Qdrant::from_url(&url).api_key(&*api_key).build()?;

    // Create the collection on first run if it doesn't exist yet. 
    let exists = client
        .list_collections()
        .await?
        .collections
        .iter()
        .any(|c| c.name == COLLECTION);

    if !exists {
        client
            .create_collection(
                CreateCollectionBuilder::new(COLLECTION)
                    .vectors_config(VectorParamsBuilder::new(1, Distance::Cosine))
            )
            .await?;
        println!("Created Qdrant collection '{COLLECTION}'.")
    }

    let mut payload: HashMap<String, Value> = HashMap::new();
    payload.insert(
        "last_ping".to_string(),
        Value { kind: Some(Kind::StringValue(now.clone())) }
    );

    client
        .upsert_points(
            UpsertPointsBuilder::new(
                COLLECTION,
                vec![PointStruct::new(POINT_ID, vec![0.0_f32], payload)]
            )
            .wait(true),
        )
        .await?;
    
    println!("✅ Qdrant - last_ping = {now}");
    Ok(())
}