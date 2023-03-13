pub use notify::RecursiveMode as RecurseMode;
use notify::{EventKind};

// TODO: check if the :: and . are right 
// TODO: directly reference to Docs of notify for EventKind and WatcherKind
// TODO: check for running mistakes

/** Determines which types of events should be passed to the hook
 * 
 * This enum, will be matched against `notify::event.kind` which are of type `notify::EventKind`, 
 * used to filter out events needed, by the user, without the need to exactly specify fine details,
 * as in EventKind, neither as general as `notify::EventKind.Any` for example, if you want to monitor
 * `Access` events only, you can do so by:
 * 
 * ```
 * Negahban{
 *    hook: Box::new(|event, _| (println!("{:#?}", event))),
 *    triggers: hashset!(
 *        EventType::Access
 *    ),
 *    ..Negahban::default()
 * }.watch();
 * ```
 * 
 * Note 1: that most deletes that perform a move to trash are considered `EventType::Modify`, not
 * `EventType::Remove`
 * 
 * Note 2: The `hashset!` macro is included in the crate for ease of use.
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

/** [WIP] Determines the type of watcher used by the `notify` crate.
 *
 * The default value is `WatcherMode::Auto`, which uses the system's native watcher, if available.
 * Other options include `WatcherMode::Poll`, which periodically polls the file system for changes,
 * and `WatcherMode::Native`, which uses one of the supported native watchers (e.g., inotify on Linux,
 * FS-Event on macOS, KQueue on BSD and optionally macOS, and Windows API on Windows).
 *
 * Note: Only change the default value if you have a good reason to do so. If you need more fine-grained
 * control over the watcher, consider using `notify` directly and take a look at `notify::RecommendedWatcher`
 * and `notify::WatcherKind`.
 */
pub enum WatcherMode {
    Poll,
    Native,
    Auto,
    // Specific(WatcherKind)
}

// pub enum WatchMode {
//     Blocking,
//     NonBlocking,
//     Async,
//     // tokio msvc and other stuff
// }

// pub enum HookType {
//     Closure(Box<dyn Fn(&Event, &DT) -> ()>),
//     Command(),
// }