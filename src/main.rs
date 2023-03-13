use negahban::Negahban;

fn main() {
    negahban::Negahban{
        hook: Box::new(|event, _| (println!("{:?}", event))),
        ..Negahban::default()
    }.run();
}
