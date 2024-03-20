use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
pub fn send_email_smtp(
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    // let cert = std::fs::read("./mail.lk-ft.ru.pem").unwrap();
    // let root_cert = Certificate::from_pem(&cert).unwrap();
    let smtppassword = std::env::var("SMTP_PASS").unwrap_or_default();
    let smtpsender = std::env::var("SMTP_SENDER").unwrap_or_default();
    let smtpserver = std::env::var("SMTP_SERVER").unwrap_or_default();
    let creds = Credentials::new(smtpsender, smtppassword);
    let mailer = SmtpTransport::relay(smtpserver.as_str())
        .unwrap()
        .credentials(creds)
        .port(465)
        .build();
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .multipart(MultiPart::alternative_plain_html(
            String::from("Hello! :)"),
            body,
        ))
        .unwrap();
    let result = mailer.send(&email);
    Ok(match result {
        Ok(_) => true,
        Err(e) => false,
    })
}
