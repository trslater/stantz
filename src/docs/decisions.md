- For BVH, I decided to go with the current structure, because:
  - It obviates the need for a specialized enum to differentiate leaves and internal nodes
  - It removes the possibility of having an AABB with two object children (every AABB is tied to at most 1 object)
  - Issues:
    - Could have a node with an object that still has children
      - This would also be a limitation of having node value be either Object or AABB