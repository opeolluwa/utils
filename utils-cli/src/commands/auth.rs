use serde::Deserialize;
use serde::Serialize;
use clap::Subcommand;
use clap::Args;

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct AuthCommands {

    #[command(subcommand)]
    command: AuthSubCommands,
}


#[derive(Debug, Subcommand, Serialize, Deserialize, Clone)]
enum AuthSubCommands {
    /// login
    Login {email:String},
    /// sign-up
    SignUp{email:String},
    /// authorize device
    AuthorizeDevice{token:String}
}


impl AuthCommands {
   pub  fn  parser(auth_command: AuthCommands){
        match auth_command.command {
            AuthSubCommands::Login{email} => AuthCommands::login(email),
            AuthSubCommands::SignUp{email} => AuthCommands::sign_up(email),
            AuthSubCommands::AuthorizeDevice{token} => AuthCommands::authorize_devive(token)
        }
    }

    pub fn login(email:String){
        println!("login {}", email)
    }

    pub fn sign_up(email:String){
        println!("sign up {}", email)
    }


    pub fn authorize_devive(token:String){
        println!("authorize device {}", token)
    }

}