use bevy::prelude::*;
use bevy_despawn_tree::*;

#[derive(Default, Component)]
struct A;

#[derive(Default, Component)]
struct B;

fn spawn_tree<T: Component + Default>(
    mut commands: Commands
) {
    commands
        .spawn_bundle((T::default(),))
        .with_children(|child_builder| {
            for _ in 0..10 {
                child_builder
                    .spawn_bundle((T::default(),))
                    .with_children(|grandchild_builder| {
                        for _ in 0..10 {
                            grandchild_builder.spawn_bundle((T::default(),));
                        }
                });
            }
        });
}

fn despawn_a_tree(
    mut commands: Commands,
    query: Query<Entity, (With<A>, With<Parent>)>
) {
    println!("\nDespawning tree A.\n");
    if let Some(child_id) = query.iter().next() {
        commands.entity(child_id).despawn_tree();
    }
}

fn count_entities(
    query_a: Query<(), With<A>>,
    query_b: Query<(), With<B>>
) {
    println!("Entities in tree A: {}", query_a.iter().len());
    println!("Entities in tree B: {}", query_b.iter().len());
}

fn main() {
    App::new()
        .add_startup_system(spawn_tree::<A>)
        .add_startup_system(spawn_tree::<B>)
        .add_system(count_entities)
        .add_system(despawn_a_tree.after(count_entities))
        .add_system_to_stage(CoreStage::PostUpdate, count_entities)
        .run();
    
}
