// Used Modules
use std::io;
use crate::utils::menu::print_menu;



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
        let charres = print_menu(
            "Choose a method of transfer",
            &[
                "[L]ink",
                "[C]opy",
                "[N]one (Default)",
            ],
        );


        let transfer_choice = {
            match charres {
                Ok(val) => val,
                Err(e) => {
                    return Err(e);
                }
            }
        };

        match transfer_choice {
            'l' | 'L' => Ok(Self::Link),
            'c' | 'C' => Ok(Self::Copy),
            _ => Ok(Self::None),
        }
    }
}
