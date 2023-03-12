// mod watch;

use std::{path::PathBuf};

use negahban::Negahban;

fn main() {
    let a = negahban::Negahban{
        hook: Box::new(|event, _| (println!("{:?}", event))),
        ..Negahban::default()
    };
    a.run()
}
