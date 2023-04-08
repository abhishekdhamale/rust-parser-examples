use regex::Regex;

#[derive(Debug)]
struct EmailAddress {
    username: String,
    domain: String,
}

fn parse_email_address(email: &str) -> Option<EmailAddress> {
    // Define the regular expression for parsing email addresses
    let re = Regex::new(r"^(?P<username>[^@\s]+)@(?P<domain>[^\s@]+)$").unwrap();

    // Use the regular expression to extract the username and domain from the email address
    match re.captures(email) {
        Some(captures) => {
            let username = captures.name("username").unwrap().as_str().to_string();
            let domain = captures.name("domain").unwrap().as_str().to_string();
            Some(EmailAddress { username, domain })
        }
        None => None,
    }
}

fn main() {
    let email1 = "alice@example.com";
    let email2 = "bob.example.com";

    match parse_email_address(email1) {
        Some(parsed) => println!("Parsed email address: {:?}", parsed),
        None => println!("Error parsing email address: {}", email1),
    }

    match parse_email_address(email2) {
        Some(parsed) => println!("Parsed email address: {:?}", parsed),
        None => println!("Error parsing email address: {}", email2),
    }
}
