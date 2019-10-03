# Hierarchical Legion Transform

This is the experimental ECS-based hierarchy branch.

## How It Works

The hierarchy is all implemented in a single ECS component, defined as:

```rust
pub struct Hierarchy {
  // The parent of this Entity, if any.
  parent: Option<Entity>,

  // The first child of this entity, if any. Children are kept in a circular
  // doubly-linked list via `siblings`. With a single child, that child will
  // have `None` for it's `siblings`.
  first_child: Option<Entity>,

  // Forms a circular doubly-linked list of siblings. None if this Entity has no
  // siblings, and both point to the same entity if it has only 1 sibling.
  siblings: Option<(Entity, Entity)>,
}
```

All mutations to the hierarchy are made via static helpers in `Hierarchy`, for
example;

```rust
// Add 2 children, `e1` and `e2` to the parent entity `parent`.
Hierarchy::set_parent_entity(&mut world, e1, parent);
Hierarchy::set_parent_entity(&mut world, e2, parent);

// Remove just `e1` from the parent.
Hierarchy::un_parent_entity(&mut world, e1);
```

## Invariant

It is invalid to delete an entity that is a part of a hierarchy without first
calling `hierarchy::un_parent_entity(...)` on it and all it's children.

This restrictions can be lifted, but probably shouldn't be as it would leave the
hierarchy in a partially-broken state until the next maintenance cycle and add
extra runtime-checks. Instead the Legion `World::delete` should be wrapped to
correctly handle hierarchies.

## Todo:

- [x] Implement a hierarchy in pure-ECS (no non-ECS allocations).
- [x] Handle adding entities to the hierarchy.
- [x] Handle removing entities from the hierarchy.
- [x] Handle tracking depth for entities that can then be used to Tag the entity
      for parallel Transform processing.
- [ ] Implement a POC Transform update system that will:
  - [ ] Walk down the dirty parts of the tree to re-tag depth changes and mark
        them as modified so they will be picked up in the next step.
  - [ ] Compute the `GlobalMatrix` for any entity with a modified
        `LocalTransform`.
  - [ ] Compute the `GlobalMatrix` for any entity that moved as a result of a
        hierarchy change.
