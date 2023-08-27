use std::time::Duration;

use clap::{Args, Subcommand};
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

// use crate::database::Database;
use crate::style::PrintColoredText;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(clap::Args, Debug, Serialize)]
pub struct EmailCommands {
    /// the email history
    #[command(subcommand)]
    pub subcommands: EmailSubCommands,
}

/// Email History sub commands  
/// utils email
#[derive(Debug, Subcommand, Serialize, Deserialize, Clone)]
pub enum EmailSubCommands {
    /// list all emails that have been sent
    History,
    /// delete email from history
    Delete { id: String },
    /// configure the SMTP parameters
    Config(ConfigOptions),
    /// send email
    New(SendOptions),
}

/// utils email new
#[derive(Serialize, Deserialize, Debug, Clone, Args)]
pub struct SendOptions {
    /// the email recipient
    #[clap(short, long, value_parser)]
    pub name: String,
    /// the recipient email address
    #[clap(short, long, value_parser)]
    pub email: String,
    /// the message content
    #[clap(short, long, value_parser)]
    pub subject: String,
    /// body/content of the message
    #[clap(short, long, value_parser)]
    pub body: Vec<String>, //
}

/// utils email config
#[derive(Args, Debug, Deserialize, Serialize, Clone)]
pub struct ConfigOptions {
    /// the SMTP username
    #[clap(short, long, value_parser)]
    pub username: String,
    /// the SMTP password
    #[clap(short, long, value_parser)]
    pub password: String,
}

/// utils email delete --id 76c310d6-2bda-58ae-8cc6-885df4fa2f99
impl EmailCommands {
    /// parse the commands
    pub fn parse(&self) {
        match &self.subcommands {
            EmailSubCommands::History => self.list(),
            EmailSubCommands::Delete { id } => self.delete(id),
            EmailSubCommands::Config(config) => self.config(config),
            EmailSubCommands::New(new) => self.send(new),
        }
    }

    fn list(&self) {
        println!("email listed");
    }

    fn delete(&self, id: &str) {
        println!("delete {}", id)
    }

    fn config(&self, config: &ConfigOptions) {
        // dave config to database
        println!("{:?}", config)
    }

    fn send(&self, data: &SendOptions) {
        //TODO get the email credentials from the database

        //TODO throw error if not found

        // if found use the template to send email
        let prompt = format!("Proceed to send email to {email}?", email = data.email);
        if Confirm::new()
            .with_prompt(prompt)
            .interact()
            .unwrap_or(false)
        {
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(120));
            pb.set_style(
                ProgressStyle::with_template("{spinner:.blue} {msg}")
                    .unwrap()
                    // For more spinners check out the cli-spinners project:
                    // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓"]),
            );
            pb.set_message("Please wait...");
            // send the email
            let email = Message::builder()
                .from("NoBody <nobody@domain.tld>".parse().unwrap())
                .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
                .to("Hei <hei@domain.tld>".parse().unwrap())
                .subject("Happy new year")
                .header(ContentType::TEXT_PLAIN)
                .body(String::from("Be happy!"))
                .unwrap();

            let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

            // Open a remote connection to gmail
            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            // Send the email
            match mailer.send(&email) {
                //TODO Save the email if it saves successfully
                Ok(_) => pb.finish_with_message("Email successfully sent"),
                Err(e) => panic!("Could not send email: {e:?}"),
            }
            // thread::sleep(Duration::from_secs(5));
        } else {
            PrintColoredText::warning("termination...")
        }
    }
}
