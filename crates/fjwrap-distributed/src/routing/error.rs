use crate::routing::types::{NodeId, ShardId};
use std::{error::Error as StdError, fmt};

#[derive(Debug, Clone)]
pub enum ConfigError {
    UnknownNode(NodeId),
    LeaderNotInReplicas(ShardId),
    NoShards,
    RangeGap,
    RangeOverlap,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::UnknownNode(node_id) => {
                write!(f, "unknown node: {:?}", node_id)
            }
            ConfigError::LeaderNotInReplicas(shard_id) => {
                write!(f, "leader not in replicas for shard: {:?}", shard_id)
            }
            ConfigError::NoShards => write!(f, "no shards defined in configuration"),
            ConfigError::RangeGap => write!(f, "gap in shard key ranges"),
            ConfigError::RangeOverlap => write!(f, "overlap in shard key ranges"),
        }
    }
}

impl StdError for ConfigError {}

#[derive(Debug, Clone)]
pub enum RouteError {
    NoShardForKey,
    ShardNotFound(ShardId),
    ShardUnavailable(ShardId),
    NoNodesAvailable(ShardId),
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteError::NoShardForKey => write!(f, "no shard found for key"),
            RouteError::ShardNotFound(shard_id) => write!(f, "shard not found: {:?}", shard_id),
            RouteError::ShardUnavailable(shard_id) => {
                write!(f, "shard unavailable: {:?}", shard_id)
            }
            RouteError::NoNodesAvailable(shard_id) => {
                write!(f, "no nodes available for shard: {:?}", shard_id)
            }
        }
    }
}

impl StdError for RouteError {}
