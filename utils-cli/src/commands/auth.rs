use crate::style::LogMessage;
use crate::utils_auth;
use clap::Args;
use clap::Subcommand;
use serde::Deserialize;
use serde::Serialize;

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct AuthCommands {
    #[command(subcommand)]
    command: AuthSubCommands,
}

#[derive(Debug, Subcommand, Serialize, Deserialize, Clone)]
enum AuthSubCommands {
    /// login
    Login { email: String },
    /// sign-up
    SignUp { email: String },
    /// authorize device
    AuthorizeDevice { token: String },
}

impl AuthCommands {
    pub fn parser(auth_command: AuthCommands) {
        match auth_command.command {
            AuthSubCommands::Login { email } => AuthCommands::login(email),
            AuthSubCommands::SignUp { email } => AuthCommands::sign_up(email),
            AuthSubCommands::AuthorizeDevice { token } => AuthCommands::authorize_devive(token),
        }
    }

    pub fn login(email: String) {
        let is_valid_email = true;
        if !is_valid_email {
            LogMessage::error("Incorrect email detected");
        } else {
        }
    }

    pub fn sign_up(email: String) {
        println!("sign up {}", email)
    }

    pub fn authorize_devive(token: String) {
        println!("authorize device {}", token)
    }
}
