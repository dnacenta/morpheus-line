use std::io::{self, Write};

/// Prompt for visible text input. Returns trimmed input.
/// If `default` is provided and the user presses Enter, returns the default.
pub fn ask(prompt: &str, default: Option<&str>) -> String {
    let display = match default {
        Some(d) => format!("  {prompt} [{d}]: "),
        None => format!("  {prompt}: "),
    };
    print!("{display}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_string();

    if trimmed.is_empty() {
        default.unwrap_or("").to_string()
    } else {
        trimmed
    }
}

/// Prompt for secret input (masked, no echo). Returns trimmed input.
pub fn ask_secret(prompt: &str) -> String {
    let display = format!("  {prompt}: ");
    rpassword::prompt_password(display).unwrap_or_default()
}

/// Yes/no confirmation prompt. Defaults to `no` when user presses Enter.
pub fn confirm(prompt: &str) -> bool {
    print!("  {prompt} [y/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
