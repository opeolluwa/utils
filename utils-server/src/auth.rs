use crate::utils_auth::utils_auth_server::UtilsAuth;


#[derive(Default)]
pub struct AuthService {}

#[tonic::async_trait]
impl UtilsAuth for AuthService {
    async fn login(
        &self,
        _request: tonic::Request<crate::utils_auth::LoginRequest>,
    ) -> std::result::Result<tonic::Response<crate::utils_auth::LoginResponse>, tonic::Status>
    {
        todo!()
    }

    async fn authorize(
        &self,
        _request: tonic::Request<crate::utils_auth::AuthorizeRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::utils_auth::AuthorizeResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn signup(
        &self,
        _request: tonic::Request<crate::utils_auth::SignupRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::utils_auth::AuthorizeResponse>,
        tonic::Status,
    > {
        todo!()
    }
}
