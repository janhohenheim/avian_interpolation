# Avian Interpolation

## Limitations

- Disables transform syncing. In practice, this means that you can *not* modify the `Transform` component of any rigid body or collider anymore.
    Use `Position` and `Rotation` instead. `Transform` is a purely aesthetic component and should not be used for physics.
- Assumes `PhysicsSchedule` is left at its default value of `FixedPostUpdate`.
- Assumes that all entities with `Position` will also have `Rotation` and vice versa.
