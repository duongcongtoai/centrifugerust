#[allow(dead_code)]
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Node<B: Broker> {
    uid: String,
    hub: Hub,
    broker: B,
    // nodes: Arc<Mutex<NodesRegistry>>,
}
struct Hub {
    shards: Vec<Arc<Mutex<SubShard>>>,
    conns: Vec<Arc<Mutex<ConnShard>>>,
    sub_locks: DashMap<i32, Mutex<i32>>,
}
struct SubShard {
    subs: DashMap<String, Client>,
}
struct ConnShard {
    conns: DashMap<String, Client>,
    users: DashMap<String, DashMap<String, ()>>, //key: user id, value: map of client ids
}
struct Client {}

struct NodesRegistry {
    nodes: DashMap<String, RemoteNode>,
}

struct RemoteNode {
    uid: String,
    Name: String,
}

pub trait Broker {}

const NUM_SUB_LOCKS: i32 = 16384;
const NUM_SUB_SHARDS: i32 = 64;

impl<B> Node<B>
where
    B: Broker,
{
    fn new(broker: B) -> Self {
        let uid = "".to_string();
        let sub_locks = DashMap::new();
        for i in 0..NUM_SUB_LOCKS {
            sub_locks.insert(i, Mutex::new(i));
        }
        let shards = vec![
            Arc::new(Mutex::new(SubShard {
                subs: DashMap::new(),
            }));
            64
        ];
        let conns = vec![
            Arc::new(Mutex::new(ConnShard {
                conns: DashMap::new(),
                users: DashMap::new(),
            }));
            64
        ];
        let hub = Hub {
            shards,
            conns,
            sub_locks,
        };
        Node { uid, hub, broker }
    }
}
