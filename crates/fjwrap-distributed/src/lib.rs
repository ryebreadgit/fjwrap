mod routing;
mod server;

use fjwrap_core::KvStore;
use fjwrap_proto::kv_service_server::KvServiceServer;
use std::net::SocketAddr;
use std::sync::Arc;

pub use fjwrap_proto::{
    KeyRange, NodeAddress, NodeId, NodeInfo, ShardConfig, ShardId, ShardStatus,
};
pub use routing::{ClusterConfig, ShardRouter, StaticRouter};
pub use server::server::KvServiceImpl;

pub async fn run_server<S, R>(
    store: Arc<S>,
    router: Arc<R>,
    node_id: u64,
    addr: SocketAddr,
) -> Result<(), tonic::transport::Error>
where
    S: KvStore + Send + Sync + 'static,
    R: ShardRouter + Send + Sync + 'static,
{
    let node_id = NodeId { id: node_id };
    let kv_service = KvServiceImpl::new(store, router, node_id);

    tracing::info!(%addr, "starting gRPC server");

    tonic::transport::Server::builder()
        .add_service(KvServiceServer::new(kv_service))
        .serve(addr)
        .await
}
