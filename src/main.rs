
use lettre::{
    message::{header::ContentType, Mailbox}, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};


fn main() {
    dotenv::dotenv().ok();

    println!("Sending emails...");
    send_email();
    println!("Emails sent!");
}

fn connect_mail_server() -> SmtpTransport {
    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");

    let creds = Credentials::new(username, password);

    SmtpTransport::relay("mail.privateemail.com")
      .unwrap()
      .credentials(creds)
      .build()
}

fn send_email() {
    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");

    let email: Message = Message::builder()
        .from(username.parse::<Mailbox>().unwrap())
        .to(username.parse::<Mailbox>().unwrap())
        .subject("Test Email")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Hello, this is a test email!"))
        .unwrap();

    let mailer: SmtpTransport = connect_mail_server();

    match mailer.send(&email) {
        Ok(_) => println!("Basic email sent!"),
        Err(error) => {
            println!("Basic email failed to send. {:?}", error);
        }
    }

}
