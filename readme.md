# Avian Interpolation

A general-purpose `Transform` interpolation plugin for fixed timesteps in [Avian Physics](https://github.com/Jondolf/avian/) for the [Bevy engine](https://bevyengine.org/).

## Usage

Add `AvianInterpolationPlugin`to your app after `PhysicsPlugins` and everything Just Works™:

```rust,ignore
App::new()
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        AvianInterpolationPlugin::default(),
    ))
    .run();
```

And that's it! The `Transform` component of all moving objects will now be interpolated after the physics simulation.
This means that the new `Transform` will be available in `Update` for rendering, spatial sound, moving your camera, etc.
The interpolation source will be their [`Position`], [`Rotation`], and, if available, [`Collider::scale()`].

Do you have any objects that should not be interpolated? Add `DisableTransformChanges` to them:

```rust,ignore
commands.spawn((
    Name::new("A wall that will never ever move"),
    RigidBody::Static,
    Collider::cuboid(10.0, 10.0, 10.0),
    DisableTransformChanges,
));
```

## Limitations

- Disables transform syncing, i.e. Avian's feature of translating `Transform` to its internal representation and vice versa.
  - If you still want to have your `Transform` changed as if you had transform syncing enabled, set `InterpolationMode::None` for that entity.
    This will use the last available physics transform as the interpolation source instead.
  - In practice, this means that you can *not* directly modify the `Transform` component of any rigid body or collider anymore.
    Use `Position` and `Rotation` instead. `Transform` is a purely aesthetic component and should not be modified for physics.
    Depending on your point of view, this is actually a feature ;)
- Assumes `PhysicsSchedule` is left at its default value of `FixedPostUpdate`.
- Assumes that all entities with `Position` will also have `Rotation` and vice versa.

## Differences to [`bevy_transform_interpolation`](https://github.com/Jondolf/bevy_transform_interpolation)

- Makes the results of the interpolation available for systems in `Update`.
- By the above features and limitations, this plugin is less memory-intensive and does fewer checks per entity.
    I didn't do any benchmarks, but it should be faster.
