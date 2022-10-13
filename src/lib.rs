use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

/// Command that walks up the bevy hierarchy tree from the given entity to the root parent entity, 
/// then call despawn_with_children_recursive on the root parent entity.
/// 
/// Equivalent to despawn_recursive if the entity has no parents.
///
/// Fails silently if the entity does not exist.
#[derive(Debug)]
pub struct DespawnTree {
    pub entity: Entity,
}

impl bevy::ecs::system::Command for DespawnTree {
    fn write(mut self, world: &mut World) {
        let mut query = world.query::<&Parent>();
        while let Ok(parent) = query.get(world, self.entity) {
            self.entity = parent.get();
        }
        despawn_with_children_recursive(world, self.entity);
    }
}

pub trait DespawnTreeEntityCommandsExt {
    fn despawn_tree(&mut self);
}

impl DespawnTreeEntityCommandsExt for EntityCommands<'_, '_, '_> {
    /// Walks up the bevy hierarchy tree from the given entity to the root parent entity, 
    /// then calls despawn_with_children_recursive on the root parent entity.
    /// 
    /// Equivalent to despawn_recursive if the entity has no parents.
    ///
    /// Fails silently if the entity does not exist.
    fn despawn_tree(&mut self) {
        let entity = self.id();
        self.commands().add(DespawnTree { entity });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use bevy::ecs::system::CommandQueue;
    use super::*;
    
    /// spawn a deep tree of 666_666 entities and then call despawn_tree on the deepest child. Check all entities removed.
    #[test]
    fn despawn_tree_a_big_tree() {
        // an innocent world
        let mut world = World::new();
        let innocent = world.spawn().id();
        let innocent_child = world.spawn().id();
        let entities_len_before = world.entities().len();

        // a wicked tree grows tall and strong
        let heretical_ancestor_id= world.spawn().id();
        let mut queue = VecDeque::from([heretical_ancestor_id]);
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);
        commands.entity(innocent).add_child(innocent_child);
        let mut count = 1;
        let mut deepest_heretical_leaf_id = heretical_ancestor_id;
        while let Some(parent_id) = queue.pop_front() {
            for _ in 0..6 {
                let child_id = commands.spawn().id();
                commands.entity(parent_id).add_child(child_id);
                queue.push_back(child_id);
                deepest_heretical_leaf_id = child_id;
                count += 1;          
            }
            if 666_666 <= count { break; }
        }
        command_queue.apply(&mut world);

        // Burn the tree from its roots, incinerate all of its sinful branches!
        let mut commands = Commands::new(&mut command_queue, &world);
        commands.entity(deepest_heretical_leaf_id).despawn_tree();
        command_queue.apply(&mut world);
        
        // Check that the tree is gone, and that no innocents were harmed in the flames.
        assert_eq!(entities_len_before, world.entities().len());
        assert!(!world.entities().contains(heretical_ancestor_id));
        assert!(!world.entities().contains(deepest_heretical_leaf_id));
        assert!(world.entities().contains(innocent));
        assert!(world.entities().contains(innocent_child));

        // peace once more
    }
}