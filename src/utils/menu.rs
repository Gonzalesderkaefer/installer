use std::io;

// Used modules and macros
use crate::FgColor;




pub fn print_menu(
    prompt: &str,
    options: &[&str],
) -> io::Result<char> {

    // First line of prompt
    let mut formated_prompt = String::from(FgColor!(Purple));
    formated_prompt.push_str(prompt);
    formated_prompt.push_str(FgColor!());
    formated_prompt.push('\n');

    // Push options to formated prompt
    for opt in options {
        formated_prompt.push_str(opt);
        formated_prompt.push('\n');
    }

    // print the prompt
    print!("{formated_prompt}");

    // Ask user for choice
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            return Err(e)
        }
    }

    // return char
    let char = buffer.as_bytes()[0];
    return Ok(char as char)
}
