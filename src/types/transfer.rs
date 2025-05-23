// Used Modules
use std::io;
use crate::FgColor;
use crate::AnsiFormat;




pub enum Transfer {
    Link,
    Copy,
    None,
}


impl Transfer {
    /// This function asks the user
    /// which transfer method to use
    /// and returns a new [Transfer]
    pub fn get_transfer() -> io::Result<Transfer> {
        println!(
            "{}{}Choose method of transfer{}\n\
            [L]ink\n\
            [C]opy\n\
            {}[N]one (Default){}",
            FgColor!(Purple),
            AnsiFormat!(Underline),
            AnsiFormat!(),
            FgColor!(Yellow),
            AnsiFormat!(),

        );

        let mut transfer_buf = String::new();
        match io::stdin().read_line(&mut transfer_buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let transfer_choice = transfer_buf.as_bytes()[0];

        match transfer_choice {
            b'L' | b'l' => Ok(Transfer::Link),
            b'C' | b'c' => Ok(Transfer::Copy),
            _ => Ok(Transfer::None),
        }
    }
}
