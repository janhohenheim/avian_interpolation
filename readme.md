# Avian Interpolation

A general-purpose [`Transform`] interpolation plugin for fixed timesteps in [Avian Physics](https://github.com/Jondolf/avian/) for the [Bevy engine](https://bevyengine.org/).

<video src="https://github.com/janhohenheim/avian_interpolation/tree/main/docs/comparison.mp4" width="50%">
A video showing the difference between enabled and disabled interpolation.
</video>
*Note: The interpolation on the left is smoother in reality, blame the video recording software ;)*

## Usage

Add the plugin to your project. Replace `3d` with `2d` if you are using Avian 2D.

```sh
cargo add avian_interpolation3d --git https://github.com/janhohenheim/avian_interpolation
```

it's not on crates.io yet because I'm waiting for a new `Avian` release, as this was made
targeting the `main` branch. This means you also need to use the `main` branch of `Avian`.
Again, replace `3d` with `2d` if you are using Avian 2D.

```sh
cargo add avian3d --git https://github.com/Jondolf/avian
```

Now, add [`AvianInterpolationPlugin`] to your app after [`PhysicsPlugins`] and everything Just Works™:

```rust,ignore
App::new()
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        AvianInterpolationPlugin::default(),
    ))
    .run();
```

And that's it! The [`Transform`] component of all moving objects will now be interpolated after the physics simulation.
This means that the new [`Transform`] will be available in [`Update`] for rendering, spatial sound, moving your camera, etc.
The interpolation source will be their [`Position`], [`Rotation`], and, if available, [`Collider::scale()`].

Do you have any objects that should not be interpolated? Add [`DisableTransformChanges`] to them:

```rust,ignore
commands.spawn((
    Name::new("A wall that will never ever move"),
    RigidBody::Static,
    Collider::cuboid(10.0, 10.0, 10.0),
    DisableTransformChanges,
));
```

## Limitations

- Disables transform syncing, i.e. Avian's feature of translating [`Transform`] to its internal representation and vice versa.
  - If you still want to have your [`Transform`] changed as if you had transform syncing enabled, set [`InterpolationMode::None`] for that entity.
    This will use the last available physics transform as the interpolation source instead.
  - In practice, this means that you can *not* directly modify the [`Transform`] component of any rigid body or collider anymore.
    Use [`Position`] and [`Rotation`] instead. [`Transform`] is a purely aesthetic component and should not be modified for physics.
    Depending on your point of view, this is actually a feature ;)
- Assumes [`PhysicsSchedule`] is left at its default value of [`FixedPostUpdate`].
- Assumes that all entities with [`Position`] will also have [`Rotation`] and vice versa.

## Differences to [`bevy_transform_interpolation`]

- [`bevy_transform_interpolation`] works with [`Transform`] in general, while this plugin works only for Avian.
- This plugin makes the results of the interpolation available for systems in [`Update`],
  which is nicer to work with than [`bevy_transform_interpolation`]'s [`PostUpdate`].
- By the above features and limitations, this plugin is less memory-intensive and does fewer checks per entity.
  I didn't do any benchmarks, but it should be faster. *Blazingly* fast, some may say.
- For most use-cases, this is should work as a drop-in replacement for [`bevy_transform_interpolation`].

## Version Compatibility

| `avian_interpolation` | `avian` | `bevy` |
|---------------|---------|-------|
| `main`       | `main` | `0.14` |

[`AvianInterpolationPlugin`]: https://github.com/janhohenheim/avian_interpolation/blob/main/src/lib.rs#L53
[`PhysicsPlugins`]: https://docs.rs/avian3d/latest/avian3d/struct.PhysicsPlugins.html
[`Transform`]: https://docs.rs/bevy/latest/bevy/transform/components/struct.Transform.html
[`Position`]: https://docs.rs/avian3d/latest/avian3d/position/struct.Position.html
[`Rotation`]: https://docs.rs/avian3d/latest/avian3d/position/struct.Rotation.html
[`DisableTransformChanges`]: https://github.com/janhohenheim/avian_interpolation/blob/main/src/lib.rs#L109
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[`PostUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PostUpdate.html
[`bevy_transform_interpolation`]: (https://github.com/Jondolf/bevy_transform_interpolation)
[`InterpolationMode::None`]: https://github.com/janhohenheim/avian_interpolation/blob/main/src/lib.rs#L99
[`PhysicsSchedule`]: https://docs.rs/avian3d/latest/avian3d/schedule/struct.PhysicsSchedule.html
[`FixedPostUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.FixedPostUpdate.html
[`Collider::scale()`]: https://docs.rs/avian3d/latest/avian3d/collision/collider/struct.Collider.html#method.scale
