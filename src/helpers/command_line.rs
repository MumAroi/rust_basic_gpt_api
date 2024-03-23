use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        stdout.execute(ResetColor).unwrap();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    stdout.execute(ResetColor).unwrap();

    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    return user_response.trim().to_string();
}

pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();

    loop {
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        println!("WARNING: You are about to run code written entirely by AI.");
        println!("Review your code and confirm you wish to continue.");

        stdout.execute(ResetColor).unwrap();

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");

        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Lets stop this project");

        stdout.execute(ResetColor).unwrap();

        let mut human_response: String = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read response");

        let human_response: String = human_response.trim().to_lowercase();

        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select '1' or '2'.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent", "Testing testing, processing something.");
    }
}
