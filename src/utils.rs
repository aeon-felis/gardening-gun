use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

pub fn sensor_events_both_ways<'a>(
    reader: &'a mut EventReader<CollisionEvent>,
) -> impl 'a + Iterator<Item = (Entity, Entity)> {
    reader
        .iter()
        .filter_map(|event| {
            if let CollisionEvent::Started(e1, e2, CollisionEventFlags::SENSOR) = event {
                Some([(*e1, *e2), (*e2, *e1)])
            } else {
                None
            }
        })
        .flatten()
}
