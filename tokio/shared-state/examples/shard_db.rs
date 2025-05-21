use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

fn hash(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

#[tokio::main]
async fn main() {
    let num_shards = 4;
    let db = new_sharded_db(num_shards);
    
    let key = "key";
    let value = vec![1, 2, 3];
    let shared_index = (hash(key) % num_shards as u64) as usize;
    let mut shard = db[shared_index].lock().unwrap();

    shard.insert(key.to_string(), value.to_vec());
}