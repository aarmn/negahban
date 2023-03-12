pub use notify::RecursiveMode as RecurseMode;
use notify::{EventKind};

#[derive(Hash, Debug, PartialEq, Eq)]
pub enum EventType {
    Access,
    Create,
    Modify,
    Remove,
    Other,
}

impl PartialEq<EventKind> for EventType {
    fn eq(&self, other: &EventKind) -> bool {
        match (self, other) {
            (EventType::Access, EventKind::Access(_)) |  
            (EventType::Create, EventKind::Create(_)) | 
            (EventType::Modify, EventKind::Modify(_)) | 
            (EventType::Remove, EventKind::Remove(_)) | 
            (EventType::Other, EventKind::Other) 
            => true,
            _ => false,
        }
    }
}

impl PartialEq<EventType> for EventKind {
    fn eq(&self, other: &EventType) -> bool {
        match (self, other) {
            (EventKind::Access(_), EventType::Access) |  
            (EventKind::Create(_), EventType::Create) | 
            (EventKind::Modify(_), EventType::Modify) | 
            (EventKind::Remove(_), EventType::Remove) | 
            (EventKind::Other, EventType::Other) 
            => true,
            _ => false,
        }
    }
}

pub enum WatchMode {
    Poll,
    Native,
    Auto,
}


// pub enum HookType {
//     Closure(Box<dyn Fn(&Event, &DT) -> ()>),
//     Command(),
// }