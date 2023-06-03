use negahban::{HookType, Negahban};

#[test]
#[ignore]
fn main() {
    let _ = Negahban{    
        hook: HookType::IndefiniteHook(Box::new(|event| {
            println!("{event:#?}");
        })),
        // recurse
        // set rest of them to defaults, if you are not sure!
        ..Negahban::default()
    }.watch();
}