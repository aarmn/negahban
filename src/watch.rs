// TODO: Test the code and write test for code
// TODO: Vector based ignore
// TODO: More imports from watchexec capabilites and ideas
// TODO: Traits setup
// TODO: Implement in the code the watch
// TODO: Document it
// TODO: Partial Equal and Equal for EventKind
// TODO: Impl more stuff
// TODO: Get rid of all bad unwraps and clones
// TODO: Handle Ctrl+C and Other Signals better
// TODO: Write a simple CLI for it with clai and stuff as a feature
// TODO: Implement WatchMode, RecurseMode and HookType
// TODO: Handle Command Running better
// TODO: Less Redundent Events

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
use crate::enums::EventType;
use crate::hashset;

// fn match_event 

fn is_included_event_type (event: &Event, event_type_hashmap: &HashSet<EventType>) -> bool {
    event_type_hashmap.into_iter().any(|event_type| event_type == &event.kind)
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Negahban<DT: 'static> // pub crate
{
    pub path: PathBuf,
    pub triggers: HashSet<EventType>,
    #[derivative(Debug="ignore")]
    pub hook: Box<dyn Fn(&Event, &DT) -> ()>,
    pub hook_data: &'static DT,
    pub ignore: Option<PathBuf>,
}

impl<DT> Negahban<DT>
{

    pub fn run(&self) {

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
                .any(|notif_path| if let Some(ignore) = &ignore {notif_path.starts_with(ignore.clone()) == false} else {true}))
            { // any path do not start with ignore pathes
                (self.hook)(&e,self.hook_data);
            }
        }
    }
}

impl Default for Negahban<Option<()>> // better
{
    fn default() -> Self {
        Self {
            path: env::current_dir().unwrap_or_default(),
            triggers: hashset![
                EventType::Create,
                EventType::Modify,
                EventType::Remove
            ],
            hook: Box::new(|_event: &Event, _none_var: &Option<_>| ()),
            hook_data: &None,
            ignore: None,
        }
    }
}
