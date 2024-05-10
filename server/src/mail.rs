use lettre::{Message, SmtpTransport, Transport}; 
use lettre::transport::smtp::{authentication::{Credentials}}; 

//aparentemente funciona
///Recibe el nombre del remitente, el nombre del destinatario, el email del destinatario, y el contenido del email.
/// El email del negocio está prefijado.
pub async fn send_email_test(name_recipient: String, email_recipient: String, content: String, subject: String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Build an email message using the builder pattern
    let name_and_email_sender = format!("{} <{}>", "Administracion de Fedeteria".to_string(), "administracion@fedeteria.com".to_string());
    let name_and_email_recipient = format!("{} <{}>", name_recipient, email_recipient);
    let body = format!("{} \n \n No contestar a este correo electronico.", content.to_string());
    let email = Message::builder()
        // Set the sender's name and email address
        .from(name_and_email_sender.parse().unwrap()) 
        // Set the recipient's name and email address
        .to(name_and_email_recipient.parse().unwrap()) 
        // Set the subject of the email
        .subject(subject) 
        // Set the body content of the email
        .body(String::from(body)) 
        .unwrap();

    // Create SMTP client credentials using username and password
    let creds = Credentials::new("c9855b99e9a768".to_string(), "7e6d5bbd275523".to_string());

    // Open a secure connection to the SMTP server using STARTTLS
    let mailer = SmtpTransport::starttls_relay("sandbox.smtp.mailtrap.io")
        .unwrap()  // Unwrap the Result, panics in case of error
        .credentials(creds)  // Provide the credentials to the transport
        .build();  // Construct the transport

    // Attempt to send the email via the SMTP transport
    match mailer.send(&email) { 
        // If email was sent successfully, print confirmation message
        Ok(_) => println!("Email sent successfully!"), 
        // If there was an error sending the email, print the error
        Err(e) => eprintln!("Could not send email: {:?}", e), 
    }

    Ok(())
}