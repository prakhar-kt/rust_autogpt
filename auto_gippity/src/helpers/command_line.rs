use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdout, stdin};


#[derive(Debug, PartialEq)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue

}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {

        let mut stdout = stdout();

        let statement_color: Color =  match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Blue,
            Self::Issue => Color::Red
        };

        // Print the agent statement in specific color

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        stdout.execute(ResetColor).unwrap();


    }
}




pub fn get_user_response(question: &str) -> String {

    // The function takes in a question as input and
    // and asks user to type his answer,
    //which is then returned by the function.
    //
    //
    // # Arguments
    // * `question` - A question to be asked from the user
    //
    //

    // Create a varibale for the output
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in a specific color 
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset Color 
    stdout.execute(ResetColor).unwrap();

    // Declare a variable for capturing user input

    let mut user_input = String::new();

    // Get the user input and store it in the variable above

    stdin()
      .read_line( &mut user_input)
      .expect("Could not read user input");

    // Return user input as String after trimmming the whitespace

    return user_input.trim().to_string();

    



} 

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn tests_print_agent_message() {

        PrintCommand::AICall.print_agent_message("Managing agent", "Testing testing processing");
    }
}