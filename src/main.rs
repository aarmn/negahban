use negahban::{Negahban};

fn main() {
    Negahban{
        // fields you want to change e.g.: 
        hook: Box::new(|event, _| (println!("{:#?}", event))),
        ..Negahban::default() // sets rest of them to default
    }.watch();
}