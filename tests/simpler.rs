use negahban::{HookType, Negahban};

#[test]
#[ignore]
fn main() {
    let mut events_count = 0;
    let _ = Negahban{
        hook: HookType::IndefiniteHook(Box::new(|event| {
            events_count += 1;
            println!("{event:#?}");
            println!("# of events occurred: {events_count:#?}");
        })),
        ignore: vec!["./target".into()],
        // set rest of them to defaults, if you are not sure!
        ..Negahban::default()
    }.watch();
}