use crate::ecs::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hierarchy {
  pub(self) parent: Option<Entity>,
  pub(self) first_child: Option<Entity>,

  // A circular linked-list of siblings, if any, as a (previous, next) tuple.
  pub(self) siblings: Option<(Entity, Entity)>,
}

// Invariants
// - An Entity with no parent has no siblings == an Entity with siblings always has a parent.
// - All link in a Hierarchy are valid, users will un-parent before deleting Entities.
// - The `Hierarchy` component is never manually deleted.
// - The self Entity can always be found by walking the next sibling until one past when it equals
//   the previous sibling (assuming the node has siblings).

impl Hierarchy {
  /// Parents (or re-parents) an `entity` to another entity `parent`.
  pub fn set_parent_entity(world: &mut World, entity: Entity, parent_entity: Entity) {
    // Make sure both the parent and entity have a Hierarchy Component.
    Hierarchy::ensure_hierarchy_present(world, parent_entity);
    Hierarchy::ensure_hierarchy_present(world, entity);

    // Un-parent the entity (if it was already parented). Seems kind of cruel. -shrug-
    Hierarchy::un_parent_entity(world, entity);

    // Set it's new parent. This is serious dark, like the darkest dark times infinity.
    (*world.get_component_mut::<Hierarchy>(entity).unwrap()).parent = Some(parent_entity);

    let first_child = (*world.get_component::<Hierarchy>(parent_entity).unwrap()).first_child;
    match first_child {
      Some(first_child) => {
        // This is not the first child of the parent. See if it's the second.
        let first_child_siblings =
          (*world.get_component::<Hierarchy>(first_child).unwrap()).siblings;

        if let Some((old_previous, _)) = first_child_siblings {
          // Not the second child as well. Wire in 'entity' to the sibling linked-list (4 links in
          // total).

          // TODO: There should be a better way to do this (maybe with a map or something).
          {
            let mut hierarchy = world.get_component_mut::<Hierarchy>(first_child).unwrap();
            (*hierarchy).siblings = Some((entity, (*hierarchy).siblings.unwrap().1));
          }
          {
            let mut hierarchy = world.get_component_mut::<Hierarchy>(old_previous).unwrap();
            (*hierarchy).siblings = Some(((*hierarchy).siblings.unwrap().0, entity));
          }
          (*world.get_component_mut::<Hierarchy>(entity).unwrap()).siblings =
            Some((old_previous, first_child));
        } else {
          // This is the second child to be added, the 4 links span the two children.
          (*world.get_component_mut::<Hierarchy>(first_child).unwrap()).siblings =
            Some((entity, entity));
          (*world.get_component_mut::<Hierarchy>(entity).unwrap()).siblings =
            Some((first_child, first_child));
        }
      }
      None => {
        // We are the first child of the parent.
        (*world.get_component_mut::<Hierarchy>(parent_entity).unwrap()).first_child = Some(entity);
        (*world.get_component_mut::<Hierarchy>(entity).unwrap()).parent = Some(parent_entity);
      }
    }
  }

  /// Un-parent an entity.
  pub fn un_parent_entity(world: &mut World, entity: Entity) {
    println!("> Check - unparent: {:?}", entity);
    // Check that the entity has a hierarchy and a parent. Clear the parent and get the old parent
    // and the entities siblings.
    let (parent, siblings) = {
      let entity_node = world.get_component_mut::<Hierarchy>(entity);

      // The entity has no Hierarchy component
      if entity_node.is_none() {
        println!("> No Hierarchy");
        return;
      }

      let mut entity_node = entity_node.unwrap();

      if !entity_node.has_parent() {
        println!("> No Parent");
        // The entity has no parent, nothing to do.
        return;
      }

      let parent = entity_node.parent.unwrap();

      // Clear the old parent
      (*entity_node).parent = None;

      (parent, entity_node.siblings)
    };

    println!("Unparent: {:?} with siblings: {:?}", entity, siblings);

    // It is assumed at this point that the parent is valid as are all links.

    // There are 2 cases for out parent:
    // - It's first_child points to another sibling, we are good to go.
    // - It's first_child points to this entity, need to shift that over to right.
    {
      let mut parent_hierarchy = world.get_component_mut::<Hierarchy>(parent).unwrap();
      if parent_hierarchy.first_child.unwrap() == entity {
        println!(
          "Shifting first_child from {:?} to {:?}",
          parent_hierarchy.first_child.unwrap(),
          siblings.map(|sibs| sibs.1)
        );
        // The second case, we need to shift the first_child to the next sibling assuming we have
        // any siblings, None otherwise.
        (*parent_hierarchy).first_child = siblings.map(|sibs| sibs.1);
      }
    }

    // There a re 3 cases for siblings:
    // - 0 other siblings (don't need to un-wire)
    // - 1 other sibling (siblings.0 == siblings.1), just set other sibling to None.
    // - 2+ other siblings, need to re-wire: left.right = right and right.left = left
    if let Some((previous, next)) = siblings {
      if previous == next {
        println!(
          "Case 2, have 1 other sibling. We are {:?}, other sibling is {:?}",
          entity, next
        );
        // Case 2, we have 1 sibling and can just set it's siblings to None
        (*world.get_component_mut::<Hierarchy>(next).unwrap()).siblings = None;
      } else {
        // Case 3, we have 2+ siblings and need to re-wire them.
        {
          let mut hierarchy = world.get_component_mut::<Hierarchy>(previous).unwrap();
          (*hierarchy).siblings = Some(((*hierarchy).siblings.unwrap().0, next));
        }
        {
          let mut hierarchy = world.get_component_mut::<Hierarchy>(next).unwrap();
          (*hierarchy).siblings = Some((previous, (*hierarchy).siblings.unwrap().1));
        }
      }
    }
  }

  pub fn has_parent(&self) -> bool {
    self.parent.is_some()
  }

  pub fn has_siblings(&self) -> bool {
    self.siblings.is_some()
  }

  pub fn has_children(&self) -> bool {
    self.first_child.is_some()
  }

  pub fn parent(&self) -> Option<Entity> {
    self.parent
  }

  pub fn children<'a>(&self, world: &'a World) -> SiblingIterator<'a> {
    SiblingIterator::new(world, self.first_child)
  }

  fn ensure_hierarchy_present(world: &mut World, entity: Entity) {
    if world.get_component::<Hierarchy>(entity).is_none() {
      world.add_component(
        entity,
        Hierarchy {
          parent: None,
          first_child: None,
          siblings: None,
        },
      );
    }
  }
}

pub struct SiblingIterator<'a> {
  world: &'a World,
  first_entity: Option<Entity>,
  cursor: Option<Entity>,
}

impl<'a> SiblingIterator<'a> {
  pub fn new(world: &'a World, first_entity: Option<Entity>) -> Self {
    SiblingIterator {
      world,
      first_entity,
      cursor: first_entity,
    }
  }
}

impl<'a> Iterator for SiblingIterator<'a> {
  type Item = Entity;

  fn next(&mut self) -> Option<Self::Item> {
    // Continue iterating until the cursor is None. Set the cursor to None explicitly if the next
    // node in the list is the first_node.
    let first_entity = self.first_entity?;
    let cursor = self.cursor?;

    // Lookup the next sibling of cursor.
    match (*self.world.get_component::<Hierarchy>(cursor).unwrap()).siblings {
      Some((_, next_sibling)) => {
        if next_sibling == first_entity {
          // The node after cursor is the one we started at, so this is our last node.
          self.cursor = None;
        } else {
          // More nodes to explore, increment the cursor forward one in the linked-list.
          self.cursor = Some(next_sibling);
        }
      }
      None => {
        // This node has no siblings, we are done iterating after returning cursor.
        self.cursor = None;
      }
    };

    Some(cursor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::LocalTransform;

  #[test]
  fn parent_one() {
    let mut world = Universe::new().create_world();
    let entities = world.insert(
      (),
      vec![(LocalTransform::identity(),), (LocalTransform::identity(),)],
    );
    let (e1, e2) = (entities[0], entities[1]);

    // Parent e2 to e1.
    Hierarchy::set_parent_entity(&mut world, e2, e1);

    // Check the hierarchy for the parent.
    let hierarchy = world.get_component::<Hierarchy>(e1).unwrap();
    assert!(!hierarchy.has_parent());
    assert!(hierarchy.has_children());
    assert_eq!(hierarchy.children(&world).collect::<Vec<_>>(), vec![e2]);

    // Check the hierarchy for the child.
    let hierarchy = world.get_component::<Hierarchy>(e2).unwrap();
    assert!(hierarchy.has_parent());
    assert!(!hierarchy.has_children());
    assert_eq!(hierarchy.parent().unwrap(), e1);
    assert_eq!(hierarchy.children(&world).collect::<Vec<_>>(), vec![]);
  }

  #[test]
  fn parent_many() {
    let mut world = Universe::new().create_world();
    let entities = world.insert(
      (),
      vec![
        (LocalTransform::identity(),),
        (LocalTransform::identity(),),
        (LocalTransform::identity(),),
        (LocalTransform::identity(),),
      ],
    );
    let (e1, e2, e3, e4) = (entities[0], entities[1], entities[2], entities[3]);

    // Parent all of [e2, e3, e4] to e1.
    Hierarchy::set_parent_entity(&mut world, e2, e1);
    Hierarchy::set_parent_entity(&mut world, e3, e1);
    Hierarchy::set_parent_entity(&mut world, e4, e1);

    // Get the hierarchy for the parent
    let hierarchy = world.get_component::<Hierarchy>(e1).unwrap();

    assert!(!hierarchy.has_parent());
    assert!(hierarchy.has_children());
    assert_eq!(
      hierarchy.children(&world).collect::<Vec<_>>(),
      vec![e2, e3, e4]
    );
  }

  #[test]
  fn remove_many_children() {
    let mut world = Universe::new().create_world();
    let entities = world.insert(
      (),
      vec![
        (LocalTransform::identity(),),
        (LocalTransform::identity(),),
        (LocalTransform::identity(),),
        (LocalTransform::identity(),),
      ],
    );
    let (parent, e1, e2, e3) = (entities[0], entities[1], entities[2], entities[3]);

    // Parent all of [e1, e2, e3] to e1.
    Hierarchy::set_parent_entity(&mut world, e1, parent);
    Hierarchy::set_parent_entity(&mut world, e2, parent);
    Hierarchy::set_parent_entity(&mut world, e3, parent);

    assert_eq!(
      world
        .get_component::<Hierarchy>(parent)
        .unwrap()
        .children(&world)
        .collect::<Vec<_>>(),
      vec![e1, e2, e3]
    );

    Hierarchy::un_parent_entity(&mut world, e1);

    assert_eq!(
      world
        .get_component::<Hierarchy>(parent)
        .unwrap()
        .children(&world)
        .collect::<Vec<_>>(),
      vec![e2, e3]
    );

    Hierarchy::un_parent_entity(&mut world, e3);

    assert_eq!(
      world
        .get_component::<Hierarchy>(parent)
        .unwrap()
        .children(&world)
        .collect::<Vec<_>>(),
      vec![e2]
    );

    Hierarchy::un_parent_entity(&mut world, e2);

    assert_eq!(
      world
        .get_component::<Hierarchy>(parent)
        .unwrap()
        .children(&world)
        .collect::<Vec<_>>(),
      vec![]
    );

    let hierarchy = world.get_component::<Hierarchy>(e1).unwrap();
    assert!(!hierarchy.has_children());
    assert_eq!(hierarchy.children(&world).collect::<Vec<_>>(), vec![]);

    // Make sure the parent was cleared on e1, e2, and e3
    assert!(!world.get_component::<Hierarchy>(e1).unwrap().has_parent());
    assert!(!world.get_component::<Hierarchy>(e2).unwrap().has_parent());
    assert!(!world.get_component::<Hierarchy>(e3).unwrap().has_parent());
  }
}
