use crate::utils_storage::utils_data_back_up_server::UtilsDataBackUp;



#[derive(Default)]
pub struct StorageService {}


#[tonic::async_trait]

impl UtilsDataBackUp for StorageService {
    async fn backup_data(
        &self,
        request: tonic::Request<crate::utils_storage::BackupDataRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::utils_storage::BackupDataResponse>,
        tonic::Status,
    > {
        todo!()
    }
}