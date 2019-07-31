mod launcher;

use console::{Term, Key, style};
use launcher::{Launcher, Commands};

fn main() {
    println!("{} {} {}", style("Syntek").blue(), style("USB").red(), style("Missile Launcher").white());
    println!("Use the {} to move, and {} to fire. To stop the current action press the same key again.",
             style("arrow keys").green(),
             style("space").green());

    let term = Term::stdout();
    let mut launcher = Launcher::new();

    loop {
        match term.read_key().unwrap() {
            Key::ArrowLeft => launcher.execute_command(Commands::LEFT),
            Key::ArrowRight => launcher.execute_command(Commands::RIGHT),
            Key::ArrowUp => launcher.execute_command(Commands::UP),
            Key::ArrowDown => launcher.execute_command(Commands::DOWN),
            Key::Char(' ') => launcher.execute_command(Commands::FIRE),
            Key::Escape => break,
            _ => continue
        }
    }
}