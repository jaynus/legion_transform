#![allow(dead_code)]
use crate::{components::*, ecs::prelude::*, math::Matrix4};

#[derive(Default)]
pub struct LocalToWorldSystem;
impl LocalToWorldSystem {
    pub fn build(&mut self) -> Box<dyn Schedulable> {
        SystemBuilder::<()>::new("LocalToWorldUpdateSystem")
            // Translation
            .with_query(<(Write<LocalToWorld>, Read<Translation>)>::query().filter(
                !component::<Parent>()
                    & !component::<Rotation>()
                    & !component::<Scale>()
                    & !component::<NonUniformScale>()
                    & (changed::<Translation>()),
            ))
            // Rotation
            .with_query(<(Write<LocalToWorld>, Read<Rotation>)>::query().filter(
                !component::<Parent>()
                    & !component::<Translation>()
                    & !component::<Scale>()
                    & !component::<NonUniformScale>()
                    & (changed::<Rotation>()),
            ))
            // Scale
            .with_query(<(Write<LocalToWorld>, Read<Scale>)>::query().filter(
                !component::<Parent>()
                    & !component::<Translation>()
                    & !component::<Rotation>()
                    & !component::<NonUniformScale>()
                    & (changed::<Scale>()),
            ))
            // NonUniformScale
            .with_query(
                <(Write<LocalToWorld>, Read<NonUniformScale>)>::query().filter(
                    !component::<Parent>()
                        & !component::<Translation>()
                        & !component::<Rotation>()
                        & !component::<Scale>()
                        & (changed::<NonUniformScale>()),
                ),
            )
            // Translation + Rotation
            .with_query(
                <(Write<LocalToWorld>, Read<Translation>, Read<Rotation>)>::query().filter(
                    !component::<Parent>()
                        & !component::<Scale>()
                        & !component::<NonUniformScale>()
                        & (changed::<Translation>() | changed::<Rotation>()),
                ),
            )
            // Translation + Scale
            .with_query(
                <(Write<LocalToWorld>, Read<Translation>, Read<Scale>)>::query().filter(
                    !component::<Parent>()
                        & !component::<Rotation>()
                        & !component::<NonUniformScale>()
                        & (changed::<Translation>() | changed::<Scale>()),
                ),
            )
            // Translation + NonUniformScale
            .with_query(
                <(
                    Write<LocalToWorld>,
                    Read<Translation>,
                    Read<NonUniformScale>,
                )>::query()
                .filter(
                    !component::<Parent>()
                        & !component::<Rotation>()
                        & !component::<Scale>()
                        & (changed::<Translation>() | changed::<NonUniformScale>()),
                ),
            )
            // Rotation + Scale
            .with_query(
                <(Write<LocalToWorld>, Read<Rotation>, Read<Scale>)>::query().filter(
                    !component::<Parent>()
                        & !component::<Translation>()
                        & !component::<NonUniformScale>()
                        & (changed::<Rotation>() | changed::<Scale>()),
                ),
            )
            // Rotation + NonUniformScale
            .with_query(
                <(Write<LocalToWorld>, Read<Rotation>, Read<NonUniformScale>)>::query().filter(
                    !component::<Parent>()
                        & !component::<Translation>()
                        & !component::<Scale>()
                        & (changed::<Rotation>() | changed::<NonUniformScale>()),
                ),
            )
            // Translation + Rotation + Scale
            .with_query(
                <(
                    Write<LocalToWorld>,
                    Read<Translation>,
                    Read<Rotation>,
                    Read<Scale>,
                )>::query()
                .filter(
                    !component::<Parent>()
                        & !component::<NonUniformScale>()
                        & (changed::<Translation>() | changed::<Rotation>() | changed::<Scale>()),
                ),
            )
            // Translation + Rotation + NonUniformScale
            .with_query(
                <(
                    Write<LocalToWorld>,
                    Read<Translation>,
                    Read<Rotation>,
                    Read<NonUniformScale>,
                )>::query()
                .filter(
                    !component::<Parent>()
                        & !component::<Scale>()
                        & (changed::<Translation>()
                            | changed::<Rotation>()
                            | changed::<NonUniformScale>()),
                ),
            )
            // Just to issue warnings: Scale + NonUniformScale
            .with_query(
                <(Read<LocalToWorld>, Read<Scale>, Read<NonUniformScale>)>::query()
                    .filter(!component::<Parent>()),
            )
            .build(move |_commands, _world, _, queries| {
                // Translation
                queries.0.par_for_each(|(mut ltw, translation)| {
                    *ltw = LocalToWorld(translation.to_homogeneous());
                });

                // Rotation
                queries.1.par_for_each(|(mut ltw, rotation)| {
                    *ltw = LocalToWorld(rotation.to_homogeneous());
                });

                // Scale
                queries.2.par_for_each(|(mut ltw, scale)| {
                    *ltw = LocalToWorld(Matrix4::new_scaling(scale.0));
                });

                // NonUniformScale
                queries.3.par_for_each(|(mut ltw, non_uniform_scale)| {
                    *ltw = LocalToWorld(Matrix4::new_nonuniform_scaling(&non_uniform_scale.0));
                });

                // Translation + Rotation
                queries.4.par_for_each(|(mut ltw, translation, rotation)| {
                    *ltw = LocalToWorld(
                        rotation
                            .to_homogeneous()
                            .append_translation(&translation.vector),
                    );
                });

                // Translation + Scale
                queries.5.par_for_each(|(mut ltw, translation, scale)| {
                    *ltw = LocalToWorld(translation.to_homogeneous().prepend_scaling(scale.0));
                });

                // Translation + NonUniformScale
                queries
                    .6
                    .par_for_each(|(mut ltw, translation, non_uniform_scale)| {
                        *ltw = LocalToWorld(
                            translation
                                .to_homogeneous()
                                .prepend_nonuniform_scaling(&non_uniform_scale.0),
                        );
                    });

                // Rotation + Scale
                queries.7.par_for_each(|(mut ltw, rotation, scale)| {
                    *ltw = LocalToWorld(rotation.to_homogeneous().prepend_scaling(scale.0));
                });

                // Rotation + NonUniformScale
                queries
                    .8
                    .par_for_each(|(mut ltw, rotation, non_uniform_scale)| {
                        *ltw = LocalToWorld(
                            rotation
                                .to_homogeneous()
                                .prepend_nonuniform_scaling(&non_uniform_scale.0),
                        );
                    });

                // Translation + Rotation + Scale
                queries
                    .9
                    .par_for_each(|(mut ltw, translation, rotation, scale)| {
                        *ltw = LocalToWorld(
                            rotation
                                .to_homogeneous()
                                .append_translation(&translation.vector)
                                .prepend_scaling(scale.0),
                        );
                    });

                // Translation + Rotation + NonUniformScale
                queries
                    .10
                    .par_for_each(|(mut ltw, translation, rotation, non_uniform_scale)| {
                        *ltw = LocalToWorld(
                            rotation
                                .to_homogeneous()
                                .append_translation(&translation.vector)
                                .prepend_nonuniform_scaling(&non_uniform_scale.0),
                        );
                    });

                // Just to issue warnings: Scale + NonUniformScale
                queries.11.iter_entities().for_each(
                    |(entity, (mut _ltw, _scale, _non_uniform_scale))| {
                        log::warn!(
                            "Entity {:?} has both a Scale and NonUniformScale component.",
                            entity
                        );
                    },
                );
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_world_transformation() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut world = Universe::new().create_world();
        let system = LocalToWorldSystem::default().build();

        let ltw = LocalToWorld::identity();
        let t = Translation::new(1.0, 2.0, 3.0);
        let r = Rotation::from_euler_angles(1.0, 2.0, 3.0);
        let s = Scale(2.0);
        let nus = NonUniformScale::new(1.0, 2.0, 3.0);

        // Add every combination of transform types.
        let translation = *world.insert((), vec![(ltw, t)]).first().unwrap();
        let rotation = *world.insert((), vec![(ltw, r)]).first().unwrap();
        let scale = *world.insert((), vec![(ltw, s)]).first().unwrap();
        let non_uniform_scale = *world.insert((), vec![(ltw, nus)]).first().unwrap();
        let translation_and_rotation = *world.insert((), vec![(ltw, t, r)]).first().unwrap();
        let translation_and_scale = *world.insert((), vec![(ltw, t, s)]).first().unwrap();
        let translation_and_nus = *world.insert((), vec![(ltw, t, nus)]).first().unwrap();
        let rotation_scale = *world.insert((), vec![(ltw, r, s)]).first().unwrap();
        let rotation_nus = *world.insert((), vec![(ltw, r, nus)]).first().unwrap();
        let translation_rotation_scale = *world.insert((), vec![(ltw, t, r, s)]).first().unwrap();
        let translation_rotation_nus = *world.insert((), vec![(ltw, t, r, nus)]).first().unwrap();

        // Run the system
        system.run(&mut world);
        system.command_buffer_mut().write(&mut world);

        // Verify that each was transformed correctly.
        assert_eq!(
            world.get_component::<LocalToWorld>(translation).unwrap().0,
            t.to_homogeneous()
        );
        assert_eq!(
            world.get_component::<LocalToWorld>(rotation).unwrap().0,
            r.to_homogeneous()
        );
        assert_eq!(
            world.get_component::<LocalToWorld>(scale).unwrap().0,
            Matrix4::new_scaling(s.0),
        );
        assert_eq!(
            world
                .get_component::<LocalToWorld>(non_uniform_scale)
                .unwrap()
                .0,
            Matrix4::new_nonuniform_scaling(&nus.0),
        );
        assert_eq!(
            world
                .get_component::<LocalToWorld>(translation_and_rotation)
                .unwrap()
                .0,
            r.to_homogeneous().append_translation(&t.vector),
        );
        assert_eq!(
            world
                .get_component::<LocalToWorld>(translation_and_scale)
                .unwrap()
                .0,
            t.to_homogeneous().prepend_scaling(s.0),
        );
        assert_eq!(
            world
                .get_component::<LocalToWorld>(translation_and_nus)
                .unwrap()
                .0,
            t.to_homogeneous().prepend_nonuniform_scaling(&nus.0),
        );
        assert_eq!(
            world
                .get_component::<LocalToWorld>(rotation_scale)
                .unwrap()
                .0,
            r.to_homogeneous().prepend_scaling(s.0)
        );
        assert_eq!(
            world.get_component::<LocalToWorld>(rotation_nus).unwrap().0,
            r.to_homogeneous().prepend_nonuniform_scaling(&nus.0)
        );
        assert_eq!(
            world
                .get_component::<LocalToWorld>(translation_rotation_scale)
                .unwrap()
                .0,
            r.to_homogeneous()
                .append_translation(&t.vector)
                .prepend_scaling(s.0)
        );
        assert_eq!(
            world
                .get_component::<LocalToWorld>(translation_rotation_nus)
                .unwrap()
                .0,
            r.to_homogeneous()
                .append_translation(&t.vector)
                .prepend_nonuniform_scaling(&nus.0)
        );
    }
}
