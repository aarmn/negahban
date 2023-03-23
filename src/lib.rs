#![doc = include_str!("../readme.md")]
// #![doc(
//     html_logo_url = "https://bevyengine.org/assets/icon.png",
//     html_favicon_url = "https://bevyengine.org/assets/icon.png"
// )]

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
    HookType,
};
pub use macros::*;