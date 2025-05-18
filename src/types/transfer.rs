// Used Modules
use std::io;



pub enum Transfer {
    Link,
    Copy,
    None,
}


impl Transfer {
    /// This function asks the user
    /// which transfer method to use
    /// and returns a new [Transfer]
    pub fn get_transfer() -> Transfer {
        println!(
            "{}Choose method of transfer\n\
            [L]ink\n\
            [C]opy\n\
            [N]one{}",
        "\x1b[93m","\x1b[0m");

        let mut transfer_buf = String::new();
        io::stdin()
            .read_line(&mut transfer_buf)
            .expect("Error reading transfer");

        let transfer_choice = transfer_buf.as_bytes()[0];

        match transfer_choice {
            b'L' | b'l' => Transfer::Link,
            b'C' | b'c' => Transfer::Copy,
            _ => Transfer::None,
        }
    }
}
