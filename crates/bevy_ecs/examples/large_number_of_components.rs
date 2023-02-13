use std::any::type_name;

use bevy_ecs::{prelude::*, world::EntityMut};

macro_rules! def_component {
    ($($name: ident),+) => {
        $(
            #[derive(Component)]
            struct $name;

            impl $name {
            }
        )+

        fn spawn_all_table_components(world: &mut World) {
            $(
                world.spawn($name);
            )+
        }
    };
}

macro_rules! def_component_sparse {
    ($($name: ident),+) => {
        $(
            #[derive(Component)]
            #[component(storage = "SparseSet")]
            struct $name;

            impl $name {
            }
        )+

        fn spawn_all_sparse_components(world: &mut World) {
            $(
                world.spawn($name);
            )+
        }
    };
}

def_component!(
    C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20,
    D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20,
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20
);

def_component_sparse!(
    S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20
);

fn main() {
    // Create a new empty world and add the event as a resource
    let mut world = World::new();

    spawn_all_table_components(&mut world);
    spawn_all_sparse_components(&mut world);

    test_bundle(&mut world, (C0,));
    test_bundle(&mut world, (C1,));
    test_bundle(&mut world, (C2,));
    test_bundle(&mut world, (C0, C1));
    test_bundle(&mut world, (C0, C2));
    test_bundle(&mut world, (C1, C2));
    test_bundle(&mut world, (C1, C2, C3));
    test_bundle_insert(&mut world, (C1, C2, C3), (C10,));
    test_bundle_insert(&mut world, (C1, C2, C3), (C11,));
    test_bundle_insert(&mut world, (C1, C2, C3), (C12,));
    test_bundle_insert(&mut world, (C1, C2, C3), (S0,));
    test_bundle_insert(&mut world, (C1, C2, C3), (S1,));

    let world_component_len = world.components().len();

    println!(
        "world bundles: {} components: {}",
        world.bundles().len(),
        world_component_len
    );
}

fn test_bundle<B: Bundle>(world: &mut World, bundle: B) -> EntityMut {
    let type_name = type_name::<B>();
    let entity = world.spawn(bundle);
    println!("{type_name}: {}", entity_debug_str(&entity),);
    entity
}

fn test_bundle_insert<B1: Bundle, B2: Bundle>(world: &mut World, bundle_1: B1, bundle_2: B2) {
    let mut entity = test_bundle(world, bundle_1);
    let type_name = type_name::<B2>();
    entity.insert(bundle_2);
    println!(" after insert {type_name}: {}", entity_debug_str(&entity));
}

fn entity_debug_str(entity: &EntityMut) -> String {
    entity.world().bundles();
    format!(
        "archetype: {:?} table: {:?} components_sparse: {} items {} bytes",
        entity.archetype().id(),
        entity.archetype().table_id(),
        entity.archetype().components_sparse_len(),
        entity.archetype().components_sparse_mem_size(),
    )
}
