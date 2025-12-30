use inquire::{Text, Select};

/// Runs the prank questionnaire
/// Returns formatted message like "Bob Blew Up!"
pub fn run_prank_ui() -> String {
    // TODO:
    // 1. Text prompt: "What is your name?"
    // 2. Select prompt: "What is the color of the sky?" with options ["Blue", "Blew"]
    // 3. Select prompt: "↑↑↑ What is the direction? ↑↑↑" with options ["Up", "Down", "Left", "Right"]
    // 4. Format and return: "{name} {color} {direction}!"
    let name = Text::new("What's your name?")
        .prompt()
        .unwrap_or_else(|_| "You".to_string());

    let color = Select::new(
        "What's the color of the sky?",
        vec!["Blew", "Blue"]
    )
    .prompt()
    .unwrap_or("Blew");

    let direction = Select::new(
        "↑↑↑ What is the direction? ↑↑↑",
        vec!["Up", "up"]
    )
    .prompt()
    .unwrap();

    format!("{} {} {}! Get rekt nerd!", name, color, direction)

}