# Stantz

## Examples

### Cornell Box

My take on the famous rendering demo scene.

Usage:

```
cargo run --example cornell_box WIDTH HEIGHT FILE
```

### Random Spheres

Generates `NUM_SPHERES` spheres with random sizes, positions, colours, and materials, and also `NUM_LIGHTS` random lights with random positions and colours. `SEED` allows recreating the same scene.

Usage:

```
cargo run --example random_spheres NUM_SPHERES NUM_LIGHTS SEED WIDTH HEIGHT FILE
```

## Roadmap

- [x] Finalize object/material coupling
- [ ] Implement BVH
- [ ] Implement reflections
- [ ] Transition to path tracing

## References

- https://www.cs.cmu.edu/~blelloch/papers/GHFB13.pdf
