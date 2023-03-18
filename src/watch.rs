// TODO: Test the code and write test for code
// TODO: Vector based ignore
// TODO: More imports from watchexec capabilities and ideas
// TODO: Traits setup
// TODO: Document it
// TODO: Impl more stuff
// TODO: Get rid of all bad unwraps and clones
// TODO: Handle Ctrl+C and Other Signals better
// TODO: Write a simple CLI for it with clai and stuff as a feature
// TODO: Implement WatchMode, RecurseMode and HookType
// TODO: Handle Command Running better
// TODO: Less redundant events
// TODO: Free up possible resources in case of OS watcher, if not freed
// TODO: Termination status return
// TODO: Use miette or sth else for better errors
// TODO: Async and Nonblocking support

use std::{
    env,
    collections::HashSet,
    fs::canonicalize,
    path::Path,
    path::PathBuf,
    time::Duration,
    sync::mpsc,
    marker::PhantomData,
};
use notify::{*, Watcher};
use derivative::Derivative;
use crate::enums::EventType;
use crate::hashset;

type NegahbanHook<INP,OUT> = Box<dyn Fn(&Event, INP) -> OUT>;

fn is_included_event_type (event: &Event, event_type_hashmap: &HashSet<EventType>) -> bool {
    event_type_hashmap.iter().any(|event_type| event_type == &event.kind)
}

/** The main struct of negahban crate
 * 
 */
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Negahban<DT: 'static = PhantomData<!>>
{
    /// Root path to be watched
    pub path: PathBuf,

    /// ['hashset'] of all the events to be watched, default is: ```hashset![EventType::Create,EventType::Modify,EventType::Remove]```
    pub triggers: HashSet<EventType>,

    /// the hook to be run on trigger events being emitted, default is a ignore Boxed closure that does nothing
    #[derivative(Debug="ignore")]
    pub hook: NegahbanHook<&'static DT, ()>, // we can return a hook channel as well

    /// data to be passed to the hook, it can be anything, default is PhantomData <!-- TODO: default can be better -->
    pub hook_data: &'static DT,

    /// ignore path <!-- TODO: should be able to ignore more than one path, can be an Enum which takes a file as well -->
    pub ignore: Option<PathBuf>, 
}

impl<DT> Negahban<DT>
{

    pub fn watch(&self) {

        let path = canonicalize(&self.path).unwrap();
        let ignore = if let Some(ignore) = &self.ignore {
            Some(canonicalize(ignore).unwrap())
        } else {
            None
        };

        // let ignore =canonicalize(ignore)
        // ignore.into_iter().map(|val| canonicalize(val).unwrap());

        let (sender, receiver) = mpsc::channel();
    
        let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
            // custom config for PollWatcher kind
            let config = Config::default()
                .with_poll_interval(Duration::from_secs(1));
            Box::new(PollWatcher::new(sender, config).unwrap())
        } else {
            // use default config for everything else
            Box::new(RecommendedWatcher::new(sender, Config::default()).unwrap())
        };
    
        watcher
            .watch(Path::new(&path), RecursiveMode::Recursive)
            .unwrap();

        let watcher_loop = receiver;
    
        // monitors events, if an event match the event type, and files/dirs are not ignored, run hook with the event
        for e in watcher_loop {
            let e = e.unwrap();
            if (is_included_event_type(&e, &self.triggers)) && // event type match 
               (e
                .clone()
                .paths
                .into_iter()
                .any(|notif_path| if let Some(ignore) = &ignore {!notif_path.starts_with(ignore.clone())} else {true}))
            { // any path do not start with ignore paths
                (self.hook)(&e,self.hook_data);
            }
        }
    }
}

impl Default for Negahban
{
    fn default() -> Self {
        Self {
            path: env::current_dir().unwrap_or_default(),
            triggers: hashset![
                EventType::Create,
                EventType::Modify,
                EventType::Remove
            ],
            hook: Box::new(|_: &Event, _: &PhantomData<_>| ()),
            hook_data: &PhantomData,
            ignore: None,
        }
    }
}
