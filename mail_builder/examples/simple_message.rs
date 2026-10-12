use mail_builder::MessageBuilder;

fn main() {
    // Build a simple text message with a single attachment
    let eml = MessageBuilder::new()
        .from(("John Doe", "john@doe.com"))
        .to("jane@doe.com")
        .subject("Hello, world!")
        .text_body("Message contents go here.")
        .attachment("image/png", "image.png", [1, 2, 3, 4].as_ref())
        .to_string()
        .unwrap();

    // Print raw message
    println!("{}", eml);
}
