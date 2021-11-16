use centrifuge::{Broker, Node};

struct GrpcApiServer<B: Broker> {
    api: Executor<B>,
}

struct Executor<B: Broker> {
    node: Node<B>,
}
