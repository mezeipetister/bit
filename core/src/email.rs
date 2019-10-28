// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Project A is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Project A.  If not, see <http://www.gnu.org/licenses/>.

use crate::error::*;
use crate::prelude::*;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email;

pub trait Email<'a> {
    fn to(&mut self, to: &'a str);
    fn subject(&mut self, subject: &'a str);
    fn message(&mut self, message: &'a str);
    fn send(&self) -> AppResult<()>;
}

pub struct EmailData<'a> {
    to: Option<&'a str>,
    subject: Option<&'a str>,
    body: Option<&'a str>,
}

pub fn new<'a>() -> impl Email<'a> {
    EmailData {
        to: None,
        subject: None,
        body: None,
    }
}

impl<'a> Email<'a> for EmailData<'a> {
    fn to(&mut self, to: &'a str) {
        self.to = Some(to);
    }
    fn subject(&mut self, subject: &'a str) {
        self.subject = Some(subject);
    }
    fn message(&mut self, message: &'a str) {
        self.body = Some(message);
    }
    fn send(&self) -> AppResult<()> {
        // Check field content not empty
        if self.to.is_none() || self.subject.is_none() || self.body.is_none() {
            return error("To, Subject and message fields need to have content.");
        }
        // Check email address contains @
        match self.to {
            Some(to) => {
                if !to.contains("@") {
                    return error(
                        "Wrong TO email format.
                         Not a valid email address.",
                    );
                }
            }
            None => (),
        }
        let email: lettre_email::Email = match lettre_email::Email::builder()
            .to(self.to.unwrap_or(""))
            .from("from")
            .subject(self.subject.unwrap_or(""))
            .text(self.body.unwrap_or(""))
            .build()
        {
            Ok(email) => email,
            Err(err) => return error("Error during creating email"),
        };

        let creds = Credentials::new("username".to_string(), "password".to_string());

        // Open a remote connection to gmail
        let mut mailer = SmtpClient::new_simple("client")
            .unwrap()
            .credentials(creds)
            .transport();

        // Send the email
        let result = mailer.send(email.into());

        match result {
            Ok(_) => return Ok(()),
            Err(_) => return error("Error while sending email."),
        }
    }
}

// TODO: Refactor to split email for production and test use.
// For test use, it should behave like a real email service,
// but just simulate the sending process.
// In production, it should communicate with the email server
// normally.
pub fn send_new_email(
    client: &str,
    username: &str,
    password: &str,
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
) -> Result<String, AppError> {
    let email: lettre_email::Email = match lettre_email::Email::builder()
        // Addresses can be specified by the tuple (email, alias)
        .to(to)
        // ... or by an address only
        .from(from)
        .subject(subject)
        .text(body)
        .build()
    {
        Ok(email) => email,
        Err(_) => return error("Error during creating email"),
    };

    let creds = Credentials::new(username.to_string(), password.to_string());

    // Open a remote connection to gmail
    let mut mailer = SmtpClient::new_simple(client)
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

    match result {
        Ok(_) => Ok("bruhaha".into()),
        Err(_) => error("Error while sending email."),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_send_email() {
        use super::*;
        // Ok, this is an idiotic test
        // With dummy data of course this should fail.
        assert_eq!(
            send_new_email(
                "smtp.gmail.com",
                "*@gmail.com",
                "*",
                "*@gmail.com",
                "*@gmail.com",
                "demo",
                "This is a demo email."
            )
            .is_ok(),
            false
        );
    }
}
