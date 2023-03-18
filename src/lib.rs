#![feature(never_type)]

/*! A simple file watcher, based on notify, designed to be fast,
 * easy-to-use and async friendly
 * 
 * # Example
 * 
 * ```
 * use negahban::Negahban;
 * 
 * fn main() {
 *     Negahban{
 *         hook: Box::new(|event, _| (println!("{:#?}", event))),
 *         ..Negahban::default()
 *     }.watch();
 * }
 * ```
 * 
 * The example above uses the default configuration to watch for file system changes, and prints out the events. 
 * It also uses `Negahban::default()` to set up the watcher with default parameters. 
 * You can customize the watcher by modifying the parameters of `Negahban` struct.
 * 
 * ## Features
 * 
 * * Watch files and directories for changes recursively.
 * * Ignore specific files and directories using `ignore` parameter.
 * * Watch multiple types of events (create, delete, modify, and more).
 * * Super easy to setup.
 * 
 * ## Usage
 * 
 * Add the following to your `Cargo.toml` file:
 * 
 * ```toml
 * [dependencies]
 * negahban = "x.x.x" # last semver instead of x.x.x
 * ```
 * 
 * and add this to your crate root:
 * 
 * ```
 * use negahban::Negahban;
 * ```
 */

mod watch;
mod enums;
mod macros;

pub use watch::{
    Negahban,
};

pub use enums::{
    // HookType,
    // WatchMode,
    EventType,
    WatcherMode,
    RecurseMode,
};
pub use macros::*;