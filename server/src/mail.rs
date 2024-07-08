use lettre::{Message, SmtpTransport, Transport}; 
use lettre::transport::smtp::{authentication::{Credentials}}; 

/// Recibe el nombre del recipiente, el mail del recipiente, el asunto del mail, y el contenido.
/// El email del negocio está prefijado.
pub fn send_email(name_recipient: String, email_recipient: String, subject: String, content: String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //return Ok(());
    // Build an email message using the builder pattern
    let name_and_email_sender = format!("{} <{}>", "Administración de Fedeteria", "administracion@fedeteria.com");
    let name_and_email_recipient = format!("{} <{}>", name_recipient, email_recipient);
    let body = format!("{} \n \n No contestar a este correo electronico.", content);
    let email = Message::builder()
        // Set the sender's name and email address
        .from(name_and_email_sender.parse().unwrap()) 
        // Set the recipient's name and email address
        .to(name_and_email_recipient.parse().unwrap()) 
        // Set the subject of the email
        .subject(subject) 
        // Set the body content of the email
        .body(body)?;

    // Create SMTP client credentials using username and password
    
    //mail Fedeteria
    //let creds = Credentials::new("c9855b99e9a768".to_string(), "7e6d5bbd275523".to_string());
    
    //mail Franco
    let creds = Credentials::new("17b761aa044b5f".to_string(), "289dd36ea1d167".to_string());
    // Open a secure connection to the SMTP server using STARTTLS
    let mailer = SmtpTransport::starttls_relay("sandbox.smtp.mailtrap.io")
        .unwrap()  // Unwrap the Result, panics in case of error
        .credentials(creds)  // Provide the credentials to the transport
        .build();  // Construct the transport

    // Attempt to send the email via the SMTP transport
    match mailer.send(&email) { 
        // If email was sent successfully, print confirmation message
        Ok(_) => Ok(()), 
        // If there was an error sending the email, print the error
        Err(e) => {
            log::error!("Could not send email: {:?}", e);
            Err(Box::new(e))
        }, 
    }
}