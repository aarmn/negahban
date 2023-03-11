// TODO: Test the code and write test for code
// TODO: Vector based ignore
// TODO: More imports from watchexec
// TODO: Traits setup
// TODO: Implement in the code the watch
// TODO: Better Namings
// TODO: Document it
// TODO: Make a sep crate out of it?
// TODO: Use hashset from the other rs file
// TODO: Any better way or name for EventKindExplicit
// TODO: Partial Equal and Equal for EventKindExplicit
// TODO: Impl more stuff
// TODO: Get rid of all bad unwraps and clones
// TODO: Handle Command Running better
// TODO: Handle Ctrl+C better
// TODO: Handle Other Signals
// TODO: More Control over watcher type (Poll or not) WatchMode and RecurseMode

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
use crate::enums::EventKind as My_EventKind;
use crate::hashset;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct WatchMan<DT: 'static> // pub crate
{
    pub path: PathBuf,
    pub triggers: HashSet<My_EventKind>,
    #[derivative(Debug="ignore")]
    pub hook: Box<dyn Fn(&Event, &DT) -> ()>,
    pub hook_data: &'static DT,
    pub ignore: Option<PathBuf>,
}

impl<DT> WatchMan<DT>
{
    pub fn new(
        path: PathBuf,
        triggers: HashSet<My_EventKind>,
        hook: Box<dyn Fn(&Event, &DT) -> ()>,
        hook_data: &'static DT,
        ignore: Option<PathBuf>,
    ) -> Self {

        let path = canonicalize(path).unwrap();
        let ignore = if let Some(ignore) = ignore {
            Some(canonicalize(ignore).unwrap())
        } else {
            None
        };
        // let ignore =canonicalize(ignore)
        // ignore.into_iter().map(|val| canonicalize(val).unwrap());

        WatchMan {
            path,
            triggers,
            hook,
            hook_data,
            ignore,
        }
    }

    pub fn run(&self) {
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
            .watch(Path::new(&self.path), RecursiveMode::Recursive)
            .unwrap();

        let watcher_loop = receiver;
    
        // monitors events, if an event match the event type, and files/dirs are not ignored, run hook with the event
        for e in watcher_loop {
            let e = e.unwrap();
            if (matches!(&self.triggers, e)) &&
               (e.clone().paths.into_iter().any(|notif_path| notif_path.starts_with(self.ignore.clone().unwrap()))) {
                (self.hook)(&e,self.hook_data);
            }
        }
    }
}

impl Default for WatchMan<Option<()>> // better
{
    fn default() -> Self {
        Self::new(
            env::current_dir().unwrap_or_default(),
            hashset![
                My_EventKind::Create,
                My_EventKind::Modify,
                My_EventKind::Remove
            ],
            Box::new(|_event: &Event, _none_var: &Option<_>| ()),
            &None,
            None,
        )
    }
}
