- For BVH, I decided to go with the current structure, because:
  - It obviates the need for a specialized enum to differentiate leaves and internal nodes
  - It removes the possibility of having an AABB with two object children (every AABB is tied to at most 1 object)
  - Issues:
    - Could have a node with an object that still has children
      - This would also be a limitation of having node value be either (Geometry, Material) tuple or AABB
- Camera is fixed at origin, and faces the negative z direction
  - This simplifies calculations and facilitates comprehension
- Ray is in its own module to avoid writing a non-unit vector to direction
  - A unit direction vector removes the need to normalize during ray casting which speeds up rendering
  - A non-unit direction vector would throw off colour calculations
- I decided to remove planes to facilitate constructing BVH

## Geometries

- The geometries are their own classes with the various traits defined on them as well as a variants in an enum
- Rationale:
  - Separate w/ traits
    - Allows specific geometries to be required without need for pattern matching in cases like BVH
  - Enum variants
    - Enum variants are easier to use and more performant where specific type is unknown
  - Traits allow functions to check for specific things objects need to do
