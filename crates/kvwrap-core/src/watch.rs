use async_channel::{Receiver, Sender};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum WatchEvent {
    Set {
        partition: String,
        key: Vec<u8>,
        value: Vec<u8>,
    },
    Delete {
        partition: String,
        key: Vec<u8>,
    },
}

impl WatchEvent {
    pub fn partition(&self) -> &str {
        match self {
            WatchEvent::Set { partition, .. } => partition,
            WatchEvent::Delete { partition, .. } => partition,
        }
    }

    pub fn key(&self) -> &[u8] {
        match self {
            WatchEvent::Set { key, .. } => key,
            WatchEvent::Delete { key, .. } => key,
        }
    }
}

struct WatchSubscription {
    partition: String,
    matcher: KeyMatcher,
    sender: Sender<WatchEvent>,
}

enum KeyMatcher {
    Exact(Vec<u8>),
    Prefix(Vec<u8>),
}

impl KeyMatcher {
    fn matches(&self, key: &[u8]) -> bool {
        match self {
            KeyMatcher::Exact(k) => key == k.as_slice(),
            KeyMatcher::Prefix(p) => key.starts_with(p),
        }
    }
}

#[derive(Clone, Default)]
pub struct WatchRegistry {
    subscribers: Arc<Mutex<Vec<WatchSubscription>>>,
}

impl WatchRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe_key(
        &self,
        partition: &str,
        key: &[u8],
        buffer: usize,
    ) -> Receiver<WatchEvent> {
        let (tx, rx) = async_channel::bounded(buffer);
        self.subscribers.lock().unwrap().push(WatchSubscription {
            partition: partition.to_string(),
            matcher: KeyMatcher::Exact(key.to_vec()),
            sender: tx,
        });
        rx
    }

    pub fn subscribe_prefix(
        &self,
        partition: &str,
        prefix: &[u8],
        buffer: usize,
    ) -> Receiver<WatchEvent> {
        let (tx, rx) = async_channel::bounded(buffer);
        self.subscribers.lock().unwrap().push(WatchSubscription {
            partition: partition.to_string(),
            matcher: KeyMatcher::Prefix(prefix.to_vec()),
            sender: tx,
        });
        rx
    }

    pub fn notify(&self, event: &WatchEvent) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|sub| {
            if sub.partition != event.partition() || !sub.matcher.matches(event.key()) {
                return true;
            }
            !sub.sender.is_closed() && sub.sender.try_send(event.clone()).is_ok()
        });
    }
}
