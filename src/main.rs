// Modules declarations
mod packages;
mod types;
mod formatting;
mod utils;


// Used modules and types

fn main() {
    //let transfer = types::transfer::Transfer::get_transfer();
    let char = utils::menu::print_menu("Choose a compositor", 
        &["[H]yprland","[R]iver","[S]way",]);
}
