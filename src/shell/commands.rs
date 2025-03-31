use alloc::string::String;
use core::str::SplitWhitespace;
use crate::{print, println};
use crate::vga_buffer::{self, Color, clear_screen};
use crate::time;

pub fn help() {
    println!("Available commands:");
    println!("  help     - Display this help message");
    println!("  echo     - Display a line of text");
    println!("  clear    - Clear the screen");
    println!("  about    - Display information about EclipseOS");
    println!("  version  - Display the current version of EclipseOS");
    println!("  help     - Displays \"Hello\" ");
    println!("  time     - Displays current time");
    println!("  disk     - Displays a list of different disk drives");
}

pub fn echo(mut args: SplitWhitespace) {
    let mut output = String::new();
    
    while let Some(arg) = args.next() {
        output.push_str(arg);
        output.push(' ');
    }
    
    println!("{}", output.trim_end());
}

pub fn clear() {
    clear_screen();
}

pub fn about() {
    vga_buffer::set_color(Color::Cyan, Color::Black);
    println!("\nEclipseOS");
    vga_buffer::set_color(Color::White, Color::Black);
    println!("A simple operating system written in Rust");
    println!("Developed as a learning project");
    println!("Type 'help' for available commands");
}

pub fn hello() {
    println!("Hello");
}

pub fn version() {
    println!("EclipseOS v0.1.0");
}
