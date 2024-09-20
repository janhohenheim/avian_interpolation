# Avian Interpolation

A general-purpose [`Transform`] interpolation plugin for fixed timesteps in [Avian Physics](https://github.com/Jondolf/avian/) for the [Bevy engine](https://bevyengine.org/).

[`examples/split_screen_comparison.rs`]:

<video src="https://github.com/user-attachments/assets/919c4809-0502-4b37-b789-261b7e9c7c30" width="50%">
A video showing the difference between enabled and disabled interpolation.
</video>

*Note: The interpolation on the left is smoother in reality, blame the video recording software ;)*

## Why do I need interpolation?

Ever had your character jitter around when making the camera follow them?
This plugin may be for you!

For a full explanation, see Bevy's [`physics_in_fixed_timestep`] example.
The short version is that on fast enough machines, your game will update its
rendered frame more often than it will update its physics simulation.
This means that sometimes e.g. your camera will be moved around without any physics
objects being updated this frame. This will lead to the physics object's movement
looking chppy and jittery, like on the right window in the video above.

There are a number of ways in which we can deal with this, and the easiest is
interpolation. By letting the visuals of the physics objects intentionally lag
a tiiiiiny bit behind, we can smoothly interpolate between the last two values, leading
to smooth and correct visuals, at the cost of the rendered objects
being behind the underlying physics objects by a bit. How much?
Well, long story short, expect the physics to be ahead of the graphics by a single digit
millisecond value. For most games, that is not noticeable at all and will just "magically"
make the game more smooth, like on the left window in the video above :)

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
        // Disabling SyncPlugin is optional, but will get you a performance boost.
        PhysicsPlugins::default().build().disable::<SyncPlugin>(),
        AvianInterpolationPlugin::default(),
    ))
    .run();
```

And that's it! The [`Transform`] component of all moving objects will now be interpolated after the physics simulation.
This means that the new [`Transform`] will be available in [`Update`] for rendering, spatial sound, moving your camera, etc.
The interpolation source will be their [`Position`] and [`Rotation`].

## Limitations

- Disables transform syncing, i.e. Avian's feature of translating [`Transform`] to its internal representation and vice versa.
  - In practice, this means that you can *not* directly modify the [`Transform`] component of any rigid body anymore.
    Use [`Position`] and [`Rotation`] instead. [`Transform`] is a purely aesthetic component and should not be modified for physics.
    Depending on your point of view, this is actually a feature ;)
  - You can still read the [`Transform`] of anything just as you would always do, if you want.
  - If you still want to have your [`Transform`] changed as if you had transform syncing enabled, set [`InterpolateTransformFields::translation`] or [`InterpolateTransformFields::rotation`] to [`InterpolationMode::Last`] for that entity.
    This will use the last available physics transform as the interpolation source instead.
- Assumes [`PhysicsSchedule`] is left at its default value of [`FixedPostUpdate`].
- Assumes that all entities with [`Position`] will also have [`Rotation`] and vice versa.
- Assumes [`RigidBody`]s will not form hierarchies with other [`RigidBody`]s.
- Assumes [`Rigidbody::Static`] objects will not move.
- Will not interpolate scales for you

## Differences to [`bevy_transform_interpolation`]

- [`bevy_transform_interpolation`] works with [`Transform`] in general, while this plugin works only for Avian.
- This plugin makes the results of the interpolation available for systems in [`Update`],
  which is nicer to work with than [`bevy_transform_interpolation`]'s [`PostUpdate`].
- By the above features and limitations, this plugin is less memory-intensive and does fewer checks per entity.
  I didn't do any benchmarks, but it should be faster. *Blazingly* fast, some may say.
- For most use-cases, this should work as a drop-in replacement for [`bevy_transform_interpolation`] as long as you
  don't mutate rigid bodies' [`Transform`]s by hand.

## Version Compatibility

| `avian_interpolation` | `avian` | `bevy` |
|---------------|---------|-------|
| `main`       | `main` | `0.14` |

[`physics_in_fixed_timestep`]: https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs
[`AvianInterpolationPlugin`]: https://github.com/janhohenheim/avian_interpolation/blob/main/src/lib.rs#L53
[`PhysicsPlugins`]: https://docs.rs/avian3d/latest/avian3d/struct.PhysicsPlugins.html
[`Transform`]: https://docs.rs/bevy/latest/bevy/transform/components/struct.Transform.html
[`Position`]: https://docs.rs/avian3d/latest/avian3d/position/struct.Position.html
[`Rotation`]: https://docs.rs/avian3d/latest/avian3d/position/struct.Rotation.html
[`RigidBody`]: https://docs.rs/avian3d/latest/avian3d/dynamics/rigid_body/enum.RigidBody.html
[`Rigidbody::Static`]: https://docs.rs/avian3d/latest/avian3d/dynamics/rigid_body/enum.RigidBody.html#variant.Static
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[`PostUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PostUpdate.html
[`bevy_transform_interpolation`]: (https://github.com/Jondolf/bevy_transform_interpolation)
[`PhysicsSchedule`]: https://docs.rs/avian3d/latest/avian3d/schedule/struct.PhysicsSchedule.html
[`FixedPostUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.FixedPostUpdate.html
[`InterpolationMode::Last`]: https://github.com/janhohenheim/avian_interpolation/blob/main/src/lib.rs#L129
[`examples/split_screen_comparison.rs`]: https://github.com/janhohenheim/avian_interpolation/blob/main/crates/avian_interpolation3d/examples/split_screen_comparison.rs
[`InterpolateTransformFields::translation`]: https://github.com/janhohenheim/avian_interpolation/blob/main/src/lib.rs#L101
[`InterpolateTransformFields::rotation`]: https://github.com/janhohenheim/avian_interpolation/blob/main/src/lib.rs#L103
[`bevy_transform_interpolation`]: (https://github.com/Jondolf/bevy_transform_interpolation)
