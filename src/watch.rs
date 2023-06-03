// TODO: Support: miette or sth else for better errors
// TODO: Support: serde if needed for data types
// TODO: Support: ctrlc, Handle Ctrl+C and Other Signals better
// TODO: Support: Setup some more traits of your own if needed?
// TODO: Control: Async and Nonblocking Hooks
// TODO: Control: Configs finer control
// TODO: Control: control over error conditions by hook
// TODO: Control: Termination status return, being able to stop the process
// TODO: Impl: New enums connect to code (WatchMode)
// TODO: Doc: Fix The Docs and the 
// TODO: Bugs: Free up possible resources in case of OS watcher, if not freed
// TODO: Bugs: Less redundant events
// TODO: Feature: Handle Command Running better, a simple CLI for it with clap and stuff as a feature, and let tokio as a feature for provider of mpsc channel
// TODO: Cleanup: Get rid of all bad unwraps and clones
// TODO: Cleanup: Impl more stuff
// TODO: Test: test it and  write test for code
// TODO: CI: setup ci stuff, auto ver bump, github action, test, better just file, ...
// TODO: Multi: More imports from watchexec capabilities and ideas
// TODO: Buffer overflow in lots of action being added to the stack of changes of it
// TODO: Thread based handle

use std::ffi::OsString;
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
use notify::{
    Watcher,
    Config,
    Event,
    PollWatcher, 
    WatcherKind,
    RecommendedWatcher,
    RecursiveMode
};
use derivative::Derivative;
use crate::enums::{EventType, HookType};
use crate::{hashset, WatcherMode, RecurseMode};

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
    pub ignore: Vec<OsString>,

    /// 
    pub watcher_mode: WatcherMode,

    /// 
    pub recurse: RecurseMode,
}

impl Negahban<'_>
{
    pub fn watch(&mut self) -> std::result::Result<(), NegahbanError> { // return sth 

        // prepare some of struct variables, to be used.
        let path = canonicalize(&self.path).map_err(NegahbanError::InvalidPath)?;

        let ignores = self.ignore.iter().map(|path| {
            PathBuf::from(&path).canonicalize()
        });

        let (sender, receiver) = mpsc::channel(); // here can we do tokio type one?

        let mut watcher: Box<dyn Watcher> = match self.watcher_mode {
            WatcherMode::Poll(config) => {
                Box::new(PollWatcher::new(sender, config).unwrap()) // it cannot fail
            },
            WatcherMode::Native => {
                if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
                    return Err(NegahbanError::NoNativeWatcherAvailable);
                }
                else {
                    Box::new(RecommendedWatcher::new(sender, Config::default()).unwrap())
                }
            },
            WatcherMode::Auto => {
                if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
                    let config = Config::default()
                        .with_poll_interval(Duration::from_secs(1))
                        .with_compare_contents(true);
                    Box::new(PollWatcher::new(sender, config).unwrap()) // TODO: why unwrapped?
                } else {
                    Box::new(RecommendedWatcher::new(sender, Config::default()).unwrap())
                }
            },
        };
    
        watcher
            .watch(Path::new(&path), self.recurse)
            .unwrap();

        let watcher_loop = receiver;
    
        // monitors events, if an event match the event type and files/dirs are not ignored, run hook with the event
        watcher_loop.iter().try_for_each(|e| { // try_for_each is used for controlflow based hooks
            match e {
                Ok(e) => {
                    if (is_included_event_type(&e, &self.triggers)) && // event type match 
                    ((e)
                        .paths
                        .iter()
                        .any(
                            |event_path| {
                                ignores.clone().all(|ignore| {
                                    match ignore {
                                        Ok(ignore) => !(event_path.starts_with(ignore)), // make it clone if ignore be used later
                                        Err(_) => true,
                                    }
                                })
                            }
                        ))
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

        Ok(())
    }
}

impl Default for Negahban<'_>
{
    fn default() -> Self {
        Self {
            path: env::current_dir().unwrap(),//_or_default(),
            triggers: hashset![
                EventType::Create,
                EventType::Modify,
                EventType::Remove
            ],
            hook: HookType::IndefiniteHook(
                Box::new(|_: &Event| ())
            ),
            ignore: vec![],
            watcher_mode: WatcherMode::Auto,
            recurse: RecursiveMode::Recursive,
        }
    }
}

// pub fn load_pathes_from_file() {
//     
// }
// pub fn load_pathes_from_globes_file() {
//     
// }
// load gitignore, vscodeignore, editorconfig ignore, and maybe env files as well, and json


/* Error for the negahbar crate */
pub enum NegahbanError {
    NoNativeWatcherAvailable,
    InvalidPath(std::io::Error),
}