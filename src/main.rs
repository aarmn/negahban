use negahban::{Negahban};

fn main() {
    let mut change_count = 0;
    Negahban{
        // fields you want to change e.g.: 
        hook: Box::new(|event| {
            change_count += 1;
            println!("{event:#?}");
            println!("# of events occurred: {change_count:#?}")
        }),
        ..Negahban::default() // sets rest of them to default
    }.watch();
}
