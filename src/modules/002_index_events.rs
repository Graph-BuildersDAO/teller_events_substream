use prost::Message;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::errors::Error;
use crate::pb::contract::v1::Events;

#[substreams::handlers::map]
fn index_events(events: Events) -> Result<Keys, Error> {
    Ok(match events.encoded_len() > 0 {
        false => Keys::default(),
        true => Keys {
            keys: vec!["events".to_string()],
        },
    })
}