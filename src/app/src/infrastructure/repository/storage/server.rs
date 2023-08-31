use alice_architecture::repository::IReadOnlyRepository;
use alice_architecture::utils::*;
use database_model::system::prelude::*;
use domain_storage::model::entity::storage_server::StorageServer;
use sea_orm::prelude::*;

use crate::infrastructure::database::SeaOrmDbRepository;

#[async_trait::async_trait]
impl IReadOnlyRepository<StorageServer> for SeaOrmDbRepository {
    async fn get_by_id(&self, uuid: &str) -> anyhow::Result<StorageServer> {
        StorageServerEntity::find_by_id::<Uuid>(uuid.parse()?)
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!(
                "There is no such storage_server with id: {uuid}"
            ))?
            .try_into()
    }

    async fn get_all(&self) -> anyhow::Result<Vec<StorageServer>> {
        unimplemented!()
    }
}
