pub use notify::RecursiveMode;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum EventKind {
    Access,
    Create,
    Modify,
    Remove,
    Other,
}

pub enum WatchMode {
    Poll,
    Native,
    Auto,
}