mod driver;

use console::{Term, Key, style};
use driver::{Launcher, Commands};

fn main() {
    println!("{} {} {}", style("Syntek").blue(), style("USB").red(),  style("Missile Launcher").white());
    println!("Use the {} to move, {} to stop and {} to fire.", style("arrow keys").green(), style("space").green(), style("enter").green());

    let term = Term::stdout();
    let launcher = Launcher::new();

    loop {
        match term.read_key().unwrap() {
            Key::ArrowLeft => launcher.execute_command(Commands::LEFT),
            Key::ArrowRight => launcher.execute_command(Commands::RIGHT),
            Key::ArrowUp => launcher.execute_command(Commands::UP),
            Key::ArrowDown => launcher.execute_command(Commands::DOWN),
            Key::Enter => launcher.execute_command(Commands::FIRE),
            //Key::Char(char::from(0x32)) => launcher.execute_command(Commands::STOP),
            Key::Escape => std::process::exit(0),
            _ => continue
        }
    }
}