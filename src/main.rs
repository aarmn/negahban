use negahban::{HookType, Negahban};

fn main() {
    let mut events_count = 0;
    Negahban{
        hook: HookType::IndefiniteHook(Box::new(|event| {
            events_count += 1;
            println!("{event:#?}");
            println!("# of events occurred: {events_count:#?}");
        })),
        // set rest of them to defaults, if you are not sure!
        ..Negahban::default()
    }.watch();
}
