// TODO: Support: miette or sth else for better errors
// TODO: Support: serde if needed for data types
// TODO: Support: ctrlc, Handle Ctrl+C and Other Signals better
// TODO: Support: Setup some more traits of your own if needed?
// TODO: Control: Async and Nonblocking Hooks
// TODO: Control: Intervals control
// TODO: Control: Configs finer control
// TODO: Control: control over error conditions by hook
// TODO: Control: Termination status return, being able to stop the process
// TODO: Control: Vector based ignore
// TODO: Impl: New enums connect to code (WatchMode, RecurseMode and HookType)
// TODO: Doc: Fix The Docs and the 
// TODO: Bugs: Free up possible resources in case of OS watcher, if not freed
// TODO: Bugs: Less redundant events
// TODO: Feature: Handle Command Running better, a simple CLI for it with clap and stuff as a feature
// TODO: Cleanup: Get rid of all bad unwraps and clones
// TODO: Cleanup: Impl more stuff
// TODO: Test: test it and  write test for code
// TODO: CI: setup ci stuff, auto ver bump, test, better just file, ...
// TODO: Multi: More imports from watchexec capabilities and ideas


use std::ops::ControlFlow;
use std::{
    env,
    collections::HashSet,
    fs::canonicalize,
    path::Path,
    path::PathBuf,
    time::Duration,
    sync::mpsc,
};
use notify::{*, Watcher};
use derivative::Derivative;
use crate::enums::{EventType, HookType};
use crate::{hashset, WatcherMode};


fn is_included_event_type (event: &Event, event_type_hashmap: &HashSet<EventType>) -> bool {
    event_type_hashmap.iter().any(|event_type| event_type == &event.kind)
}

/** The main struct of negahban crate
 * 
 */
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Negahban<'negahban>
{
    /// Root path to be watched
    pub path: PathBuf,

    /// ['hashset'] of all the events to be watched, default is: ```hashset![EventType::Create,EventType::Modify,EventType::Remove]```
    pub triggers: HashSet<EventType>,

    /// the hook to be run on trigger events being emitted, default is a ignore Boxed closure that does nothing
    #[derivative(Debug="ignore")]
    pub hook: HookType<'negahban>, // Maybe a termination var in B can be good

    /// ignore path <!-- TODO: should be able to ignore more than one path, can be an Enum which takes a file as well -->
    pub ignore: Option<PathBuf>, 

    /// 
    pub watcher_mode: WatcherMode,
}

impl Negahban<'_>
{
    fn choose_native_watcher() -> Option<WatcherKind> {
        match env::consts::OS {
            "windows" => {
                Some(WatcherKind::ReadDirectoryChangesWatcher)
            }
            "android" | "linux" => {
                Some(WatcherKind::Inotify)
            }
            "macos" => {
                Some(WatcherKind::Fsevent)
            }
            "ios" | "bsd" | "freebsd" | "openbsd" | "netbsd" | "dragonfly" => {
                Some(WatcherKind::Kqueue)
            }
            _ | "solaris" => {
                None
            }
        }
    }

    pub fn watch(&mut self) {

        // prepare some of struct variables, to be used.
        let path = canonicalize(&self.path).unwrap();
        let ignore = if let Some(ignore) = &self.ignore {
            Some(canonicalize(ignore).unwrap())
        } else {
            None
        };

        // TODO: multipath ignore support
        // let ignore =canonicalize(ignore)
        // ignore.into_iter().map(|val| canonicalize(val).unwrap());

        let (sender, receiver) = mpsc::channel();

        let mut watcher: Box<dyn Watcher> = match self.watcher_mode {
            WatcherMode::Poll(duration) => {
                todo!()
                // .with_compare_contents(compare_contents);
            },
            WatcherMode::Native => {
                todo!()
            },
            WatcherMode::Auto => {
                if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
                    let config = Config::default()
                        .with_poll_interval(Duration::from_secs(1));
                    // TODO: check configs, make intevrals user controlable
                    Box::new(PollWatcher::new(sender, config).unwrap()) // TODO: why unwrapped?
                } else {
                    // TODO: check configs
                    Box::new(RecommendedWatcher::new(sender, Config::default()).unwrap())
                }
            },
            WatcherMode::Specific(watcher_kind, config) => {
                // Box::new(
                //     (Self::choose_native_watcher(watcher_kind).unwrap())
                //     ::new(sender, Config::default()).unwrap()
                // )
                todo!()
            },
        };
    
        watcher
            .watch(Path::new(&path), RecursiveMode::Recursive)
            .unwrap();

        let watcher_loop = receiver;
    
        // monitors events, if an event match the event type and files/dirs are not ignored, run hook with the event
        watcher_loop.iter().try_for_each(|e| {
            match e {
                Ok(e) => {
                    if (is_included_event_type(&e, &self.triggers)) && // event type match 
                    (e
                        .clone()
                        .paths
                        .into_iter()
                        .any(|notif_path| if let Some(ignore) = &ignore {!notif_path.starts_with(ignore.clone())} else {true}))
                    { // any path do not start with ignore paths
                        match &mut self.hook {
                            HookType::ControledHook(hook) => return hook(&e),
                            HookType::IndefiniteHook(hook) => hook(&e),
                        }
                    }
                    return ControlFlow::Continue(())
                }
                Err(err) => {
                    eprintln!("{err:#?}");
                    return ControlFlow::Continue(())
                }
            }
        });
    }
}

impl Default for Negahban<'_>
{
    fn default() -> Self {
        Self {
            path: env::current_dir().unwrap_or_default(),
            triggers: hashset![
                EventType::Create,
                EventType::Modify,
                EventType::Remove
            ],
            hook: HookType::IndefiniteHook(
                Box::new(|_: &Event| ())
            ),
            ignore: None,
            watcher_mode: WatcherMode::Auto,
        }
    }
}
