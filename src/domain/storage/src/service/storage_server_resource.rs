use alice_architecture::utils::*;

use crate::model::entity::StorageServer;

/// Manage regions, avz(available zone)s, servers and queues.
#[async_trait]
pub trait StorageServerResourceService: Send + Sync {
    /// Get co system default storage server.
    async fn default_file_storage_server(&self) -> Anyhow<StorageServer>;
}
