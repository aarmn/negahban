use std::{ops::ControlFlow, time::Duration};

pub use notify::RecursiveMode as RecurseMode;
use notify::{EventKind, Event, WatcherKind, Config};

// TODO: check for running mistakes
// TODO: doc bettering of syntax (bevy inspired) 

type ControledHook<'time,Endian,Step> = Box<dyn FnMut(&Event) -> ControlFlow<Endian,Step> + 'time>;
type IndefiniteHook<'time> = Box<dyn FnMut(&Event) -> () + 'time>;

/** Determines which types of events should be passed to the hook
 * 
 * This enum, will be matched against [`notify::event::Event::kind`] which are of type [`notify::EventKind`], 
 * used to filter out events needed, by the user, without the need to exactly specify fine details,
 * as in EventKind, neither as general as [`notify::EventKind::Any`] for example, if you want to monitor
 * [`notify::EventKind::Modify`] events only, you can do so by:
 * 
 * ```
 * use negahban::{Negahban,hashset,EventType};
 * 
 * let negahban = Negahban{
 *     triggers: hashset!(EventType::Access),
 *     ..Negahban::default()
 * };
 * ```
 * 
 * Note 1: that most deletes that perform a move to trash are considered
 * [`notify::EventKind::Modify`] and not [`notify::EventKind::Remove`]
 * 
 * Note 2: The [`hashset`] macro is included in the crate for ease of use.
 */
#[derive(Hash, Debug, PartialEq, Eq)]
pub enum EventType {
    Access,
    Create,
    Modify,
    Remove,
    Other,
}

impl PartialEq<EventKind> for EventType {
    fn eq(&self, other: &EventKind) -> bool {
        match (self, other) {
            (EventType::Access, EventKind::Access(_)) |  
            (EventType::Create, EventKind::Create(_)) | 
            (EventType::Modify, EventKind::Modify(_)) | 
            (EventType::Remove, EventKind::Remove(_)) | 
            (EventType::Other, EventKind::Other) 
            => true,
            _ => false,
        }
    }
}

impl PartialEq<EventType> for EventKind {
    fn eq(&self, other: &EventType) -> bool {
        match (self, other) {
            (EventKind::Access(_), EventType::Access) |  
            (EventKind::Create(_), EventType::Create) | 
            (EventKind::Modify(_), EventType::Modify) | 
            (EventKind::Remove(_), EventType::Remove) | 
            (EventKind::Other, EventType::Other) 
            => true,
            _ => false,
        }
    }
}

/** Determines the type of watcher used by the [`notify`] crate.
 *
 * The recommended value is [`WatcherMode::Auto`], which uses the system's native watcher, if available.
 * Other options include [`WatcherMode::Poll`], which periodically polls the file system for changes,
 * and [`WatcherMode::Native`], which uses one of the supported native watchers. to explain some:
 *  - inotify on Linux,
 *  - FS-Event on macOS
 *  - KQueue on BSD and optionally macOS
 *  - Windows API on Windows
 * 
 * Note: Only change the default value if you have a **good** reason to do so. If you need more fine-grained
 * control over the watcher, consider using [`notify`] directly or [`WatcherMode::Specific`] and take a look
 * at [`notify::RecommendedWatcher`] and [`notify::WatcherKind`].
 */
#[derive(Hash, Debug, PartialEq, Eq)]
pub enum WatcherMode {
    /// Recommended: If a implemented native watcher is available on system, uses it.
    /// Otherwise, uses a PollWatcher.
    Auto,

    /// Use if you specifically want a PollWatcher, even if a native watcher is available.
    /// This would be marginally slower and more cpu intensive than a native watcher.
    Poll(Duration), // TODO: default duration??

    /// Not Recommended: Use if you specifically want a native watcher, it would **panic** if cannot be used.
    Native,

    /// **Not Recommended:** Forcing to use a specific watcher and **panics** if it is not available.
    Specific(WatcherKind, Config),
}

/// Determines the type of hook to be called on the chosen events occurrence
pub enum HookType<'Time> {
    /// Used to carry a hook, which return a callback, able to terminate the watcher 
    /// cycle; but has to return a [`ControlFlow`][`std::ops::ControlFlow`]
    ControledHook(ControledHook<'Time,(),()>), // TODO: should be generic over control flow types?

    /// Used to carry an indefinite hook, which after running there is no way to terminate,
    /// unless you run it in a thread and terminate it by terminating the thread.
    IndefiniteHook(IndefiniteHook<'Time>),
}

// /** 
//  * 
//  */
// #[derive(Hash, Debug, PartialEq, Eq)]
// pub enum WatchMode {
//     Blocking,
//     NonBlocking,
//     Async,
//     // tokio msvc and other stuff
// }