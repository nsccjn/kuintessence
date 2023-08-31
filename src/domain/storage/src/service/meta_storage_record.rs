use alice_architecture::utils::*;

use crate::model::vo::{
    record::{RecordFileMeta, RecordFileStorage},
    HashAlgorithm,
};

/// Record file_metadata and file_storage.
#[async_trait]
pub trait MetaStorageService: Send + Sync {
    /// Record uploaded file into file_metadata and file_storage.
    async fn record_meta_and_storage(
        &self,
        meta_id: Uuid,
        file_meta_info: RecordFileMeta,
        file_storage_info: RecordFileStorage,
        user_id: Option<Uuid>,
    ) -> Anyhow;

    /// Look up file_metadata to judge whether the same hash file is uploaded.
    /// If satisfy, return meta_id.
    async fn satisfy_flash_upload(
        &self,
        hash: &str,
        hash_algorithm: &HashAlgorithm,
    ) -> Anyhow<Option<Uuid>>;

    /// Get server_url by storage_server_id and meta_id.
    async fn get_server_url(&self, storage_server_id: Uuid, meta_id: Uuid) -> Anyhow<String>;
}
