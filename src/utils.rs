use bevy::ecs::query::{ReadOnlyWorldQuery, WorldQuery};
use bevy::prelude::*;

pub struct EntityMatcher<const N: usize> {
    entities: [Entity; N],
    num_matched: usize,
}

pub trait GetForEntity {
    type Output;

    fn check(&self, entity: Entity) -> bool;

    fn get(self, entity: Entity) -> Option<Self::Output>;
}

impl<'a, Q: WorldQuery, F: ReadOnlyWorldQuery> GetForEntity for &'a Query<'_, '_, Q, F> {
    type Output = <<Q as WorldQuery>::ReadOnly as WorldQuery>::Item<'a>;

    fn check(&self, entity: Entity) -> bool {
        self.contains(entity)
    }

    fn get(self, entity: Entity) -> Option<Self::Output> {
        self.get(entity).ok()
    }
}

impl<'a, Q: WorldQuery, F: ReadOnlyWorldQuery> GetForEntity for &'a mut Query<'_, '_, Q, F> {
    type Output = <Q as WorldQuery>::Item<'a>;

    fn check(&self, entity: Entity) -> bool {
        self.contains(entity)
    }

    fn get(self, entity: Entity) -> Option<Self::Output> {
        self.get_mut(entity).ok()
    }
}

impl<const N: usize> EntityMatcher<N> {
    pub fn new(entities: [Entity; N]) -> Self {
        Self {
            entities,
            num_matched: 0,
        }
    }

    pub fn get<G: GetForEntity>(&mut self, query: G) -> Option<<G as GetForEntity>::Output> {
        let rest = &mut self.entities[self.num_matched..];
        let (index, &entity) = rest
            .iter()
            .enumerate()
            .find(|(_, entity)| query.check(**entity))?;
        rest.swap(0, index);
        query.get(entity)
    }
}
