use std::time::Duration;

use clap::{Args, Subcommand};
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};
use lettre::message::{header, MultiPart, SinglePart};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::database::{self, SmtpCredentials};
// use crate::database::Database;
use crate::style::PrintColoredText;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(clap::Args, Debug, Serialize)]
pub struct EmailCommands {
    /// the email recipient
    #[clap(short, long, value_parser)]
    pub name: Option<String>,
    /// the recipient email address
    #[clap(short, long, value_parser)]
    pub email: Option<String>,
    /// the message content
    #[clap(short, long, value_parser)]
    pub subject: Option<String>,
    /// body/content of the message
    #[clap(short, long, value_parser)]
    pub body: Option<Vec<String>>, //
    /// the email history
    #[command(subcommand)]
    pub subcommands: Option<EmailSubCommands>,
}

struct SendOptions {
    /// the email recipient
    pub name: String,
    /// the recipient email address
    pub email: String,
    /// the message content
    pub subject: String,
    /// body/content of the message
    pub body: Vec<String>, //
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
    pub async fn parse(&self) {
        if let Some(command) = &self.subcommands {
            match command {
                EmailSubCommands::History => self.list(),
                EmailSubCommands::Delete { id } => self.delete(id),
                EmailSubCommands::Config(config) => self.config(config),
            }
        } else {
            macro_rules! check_required_field {
                ($self:expr, $field:ident) => {
                    if let Some($field) = $self.$field.clone() {
                        $field
                    } else {
                        PrintColoredText::warning(&format!("{} is required", stringify!($field)));
                        return;
                    }
                };
            }

            let name = check_required_field!(self, name);
            let email = check_required_field!(self, email);
            let subject = check_required_field!(self, subject);
            let body = check_required_field!(self, body);

            self.send(&SendOptions {
                name,
                email,
                subject,
                body,
            })
            .await
        }
    }

    fn list(&self) {
        println!("email listed");
    }

    fn delete(&self, id: &str) {
        println!("delete {}", id)
    }

    fn config(&self, config: &ConfigOptions) {
        // save config to database
        println!("{:?}", config)
    }

    async fn send(&self, data: &SendOptions) {
        /* get the email credentials from the database
         * if not found, prompt the user to enter the credentials
         */
        let SmtpCredentials {
            smtp_username,
            smtp_password,
        } = database::SmtpCredentials::fetch().await;

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
            let recipient = format!("{}<{}>", data.name, data.email);

            // build the template
            let mut email_body = handlebars::Handlebars::new();
            let source = include_str!("../assets/email.hbs");
            email_body
                .register_template_string("email", source)
                .expect("error reading template file");
            let  email_body = email_body.render(
                "email",
                &json!({"email":data.email, "body": data.body, "subject":data.subject, "recipient":data.name}),
            ).ok();

            let Some(html) = email_body else {
                pb.finish_with_message(
                    "Oops! An error was encountered while rendering the email. PLease retry!",
                );
                return;
            };

            let email = Message::builder()
                .from("Adeoye Adefemi <adefemiadeoye@yahoo.com>".parse().unwrap())
                .reply_to("Adeoye Adefemi <adefemiadeoye@yahoo.com>".parse().unwrap())
                .to(recipient.parse().unwrap())
                .subject(&data.subject)
                .multipart(
                    MultiPart::alternative() // This is composed of two parts.
                        .singlepart(
                            SinglePart::builder()
                                .header(header::ContentType::TEXT_PLAIN)
                                .body(String::from("Oops! An error was encountered while rendering the email. PLease retry!")), // Every message should have a plain text fallback.
                        )
                        .singlepart(
                            SinglePart::builder()
                                .header(header::ContentType::TEXT_HTML)
                                .body(html),
                        ),
                ).ok();

            let Some(email) = email else {
                pb.finish_with_message(
                    "Oops! An error was encountered while parsing the email. Please retry!",
                );
                return;
            };

            let credentials = Credentials::new(smtp_username, smtp_password);

            // Open a remote connection to gmail
            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(credentials)
                .build();

            // Send the email
            match mailer.send(&email) {
                //TODO Save the email if it saves successfully
                Ok(_) => pb.finish_with_message("Email successfully sent"),
                Err(e) => panic!("Could not send email: {e:?}"),
            }
        } else {
            PrintColoredText::warning("termination...")
        }
    }
}
