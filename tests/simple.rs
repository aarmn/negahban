use negahban::{HookType, Negahban, hashset};
use std::env;
use negahban::EventType;
use std::ops::ControlFlow;

#[test]
#[ignore]
fn main() {
    let mut events_count = 0;
    let _ = Negahban{
        path: env::current_dir().unwrap(),
        triggers: hashset!{
            EventType::Create,
            EventType::Access
        },
        // watcher_mode: WatcherM,
        hook: HookType::ControledHook(Box::new(|event| {
            events_count += 1;
            if event.kind == EventType::Create {
                return ControlFlow::Break(());
            }
            println!("{event:#?}");
            println!("# of events occurred: {events_count:#?}");
            ControlFlow::Continue(())
        })),
        ignore: vec!["./target".into(), "./out".into()],
        ..Negahban::default()
    }.watch();
}