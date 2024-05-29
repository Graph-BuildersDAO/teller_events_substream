#[path = "001_map_events.rs"]
mod map_events;

#[path = "002_index_events.rs"]
mod index_events;

pub use map_events::map_events;
pub use index_events::index_events;