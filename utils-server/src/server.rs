use crate::proto::utils_auth::utils_auth_server::UtilsAuth;

#[derive(Default)]
pub struct AuthCliServer {}

#[tonic::async_trait]
impl UtilsAuth for AuthCliServer {
    async fn login(
        &self,
        _request: tonic::Request<crate::proto::utils_auth::LoginRequest>,
    ) -> std::result::Result<tonic::Response<crate::proto::utils_auth::LoginResponse>, tonic::Status>
    {
        todo!()
    }

    async fn authorize(
        &self,
        _request: tonic::Request<crate::proto::utils_auth::AuthorizeRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::proto::utils_auth::AuthorizeResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn signup(
        &self,
        _request: tonic::Request<crate::proto::utils_auth::SignupRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::proto::utils_auth::AuthorizeResponse>,
        tonic::Status,
    > {
        todo!()
    }
}
