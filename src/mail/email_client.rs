use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use std::time::SystemTime;

pub fn connect_mail_server() -> SmtpTransport {
    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");

    let creds = Credentials::new(username, password);

    SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build()
}

pub fn send_email(boot_time: String) {
    println!("Sending emails...");

    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");

    let email: Message = Message::builder()
        .from(username.parse::<Mailbox>().unwrap())
        .to(username.parse::<Mailbox>().unwrap())
        .subject("Device boot on")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(format!("Device boot on {:?}!", boot_time)))
        .unwrap();

    let mailer: SmtpTransport = connect_mail_server();

    match mailer.send(&email) {
        Ok(_) => println!("Basic email sent!"),
        Err(error) => {
            println!("Basic email failed to send. {:?}", error);
        }
    }
    println!("Emails sent!");
}
