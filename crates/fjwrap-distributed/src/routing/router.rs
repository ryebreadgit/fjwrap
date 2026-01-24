use crate::routing::{
    error::RouteError,
    traits::{ClusterConfigExt, KeyRangeExt, RouteResult, ShardRouter},
    types::{ClusterConfig, ShardId, ShardStatus},
};
use prost::Message;
use std::sync::Arc;

#[derive(Clone)]
pub struct StaticRouter {
    config: Arc<ClusterConfig>,
    sorted_shards: Arc<Vec<ShardId>>,
}

impl StaticRouter {
    pub fn new(config: ClusterConfig) -> Self {
        let mut sorted_shards: Vec<ShardId> = config
            .shards
            .iter()
            .filter_map(|shard| shard.id.clone())
            .collect();
        sorted_shards.sort_by_key(|shard_id| shard_id.id);
        StaticRouter {
            config: Arc::new(config),
            sorted_shards: Arc::new(sorted_shards),
        }
    }

    pub fn from_json_str(json_str: &str) -> Result<Self, serde_json::Error> {
        let config: ClusterConfig = serde_json::from_str(json_str)?;
        Ok(StaticRouter::new(config))
    }

    pub fn from_proto_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        let config = ClusterConfig::decode(bytes)?;
        Ok(StaticRouter::new(config))
    }
}

impl ShardRouter for StaticRouter {
    fn route(&self, partition: &str, key: &[u8]) -> RouteResult<ShardId> {
        let shard_key = self.shard_key(partition, key);

        for shard in &self.config.shards {
            if let Some(range) = &shard.range {
                if range.contains(&shard_key) {
                    if shard.status() == ShardStatus::Active {
                        return Ok(shard.id.clone().unwrap());
                    }
                }
            }
        }

        Err(RouteError::NoShardForKey)
    }

    fn shard_nodes(&self, shard_id: ShardId) -> RouteResult<Vec<String>> {
        let shard = self
            .config
            .get_shard(&shard_id)
            .ok_or(RouteError::ShardNotFound(shard_id.clone()))?;

        let mut nodes = Vec::new();
        for node_id in &shard.replicas {
            let node = self
                .config
                .get_node(node_id)
                .ok_or(RouteError::ShardUnavailable(shard_id.clone()))?;
            if let Some(address) = &node.address {
                nodes.push(format!("{}:{}", address.host, address.port));
            }
        }

        if nodes.is_empty() {
            return Err(RouteError::NoNodesAvailable(shard_id));
        }

        Ok(nodes)
    }

    fn config_version(&self) -> RouteResult<u64> {
        Ok(self.config.version)
    }
}
