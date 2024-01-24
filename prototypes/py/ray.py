from __future__ import annotations
from contextlib import suppress

import numpy as np
from PIL import Image


def main():
    ground_height = -4
    scene = [PlaneGeometry((0, 1, 0), ground_height,
                           Material(1, 0, 0, (1, 1, 1)))]
    seed_offset = 10
    for seed_base in range(seed_offset, 10 + seed_offset):
        diffusion = np.random.default_rng(seed_base).uniform(0, 1)
        shininess = np.random.default_rng(seed_base).integers(1, 100, 1)[0]
        color = (
            np.random.default_rng(3*seed_base).uniform(0, 1),
            np.random.default_rng(3*seed_base + 1).uniform(0, 1),
            np.random.default_rng(3*seed_base + 2).uniform(0, 1))

        material = Material(diffusion, 1 - diffusion, shininess, color)

        radius = np.random.default_rng(seed_base).uniform(0.1, 1.5)
        center = (
            np.random.default_rng(3*seed_base).uniform(-8, 8),
            np.random.default_rng(3*seed_base + 1).uniform(ground_height, 2),
            np.random.default_rng(3*seed_base + 2).uniform(-1, 1))

        if (center[1] - radius) < ground_height:
            center = (center[0], radius + ground_height, center[2])

        scene.append(SphereGeometry(center, radius, material))

    lights = (
        Light(
            position=(-10, 5, 5),
            color=(1, 0, 0)),
        Light(
            position=(10, 5, 5),
            color=(0, 0, 1)))

    camera = Camera(
        origin=(0, 0, 10),
        fov=45,
        focal_length=9)

    render(scene, lights, camera, 640, 480, 2)
    # render(scene, lights, camera, 100, 50, 1)


class Material:
    def __init__(self, diffusion, specularity, shininess, color) -> None:
        self.diffusion = diffusion
        self.specularity = specularity
        self.shininess = shininess
        self.color = np.array(color, dtype=np.float32)


class SphereGeometry:
    def __init__(self, center, radius, material):
        self.center = np.array(center, dtype=np.float32)
        self.radius = radius
        self.material = material

    def intersect_ray(self, ray):
        a = np.dot(ray.direction, ray.direction)
        b = 2*np.dot((ray.origin - self.center), ray.direction)
        c = np.dot(ray.origin - self.center, ray.origin -
                   self.center) - self.radius**2

        discriminant = b**2 - 4*a*c

        # Misses
        if discriminant < 0:
            raise ValueError("doesn't intersect")

        first_term = -b/2/a

        # Just grazes surface
        if discriminant == 0:
            return first_term

        second_term = np.sqrt(discriminant)/2/a

        # Enter and exit wounds
        t1, t2 = first_term + second_term, first_term - second_term

        return np.minimum(t1, t2)

    def normal(self, point_on_surface):
        normal = point_on_surface - self.center
        return normal/np.linalg.norm(normal)


class PlaneGeometry:
    def __init__(self, normal, offset, material) -> None:
        self._normal = np.array(normal, dtype=np.float32)
        self.offset = offset
        self.material = material

    def intersect_ray(self, ray):
        shmoop = np.dot(self._normal, ray.direction)

        if shmoop == 0:
            raise ValueError()

        t = np.dot(self._normal, ray.origin) + self.offset/shmoop

        if t <= 0:
            raise ValueError()

        return t

    def normal(self, point_on_surface):
        return self._normal


class Ray:
    def __init__(self, origin, direction, color):
        self.origin = np.array(origin, dtype=np.float32)
        self.direction = np.array(direction, dtype=np.float32)
        self.color = np.array(color, dtype=np.float32)

    def point(self, t):
        return self.origin + t*self.direction


class Light:
    def __init__(self, position, color):
        self.position = np.array(position, dtype=np.float32)
        self.color = np.array(color, dtype=np.float32)


class Camera:
    def __init__(self, origin, fov, focal_length):
        self.origin = np.array(origin, dtype=np.float32)
        self.fov = fov
        self.focal_length = focal_length


ambient_light = np.array((0.1, 0.1, 0.1), dtype=np.float32)


def cast_ray(scene, lights, ray):
    # TODO: Replace with deque
    objs = []
    for _ in range(1):
        ts = []
        for obj in scene:
            with suppress(ValueError):
                ts.append((obj.intersect_ray(ray), obj))

        if ts:
            t, obj = min(ts)
            intersection = ray.point(t)

            normal = obj.normal(intersection)
            # TODO: Reflections
            # normal_component = np.dot(ray.direction, normal)*normal
            # perp_component = ray.direction - normal_component
            # bounce_dir = perp_component - normal_component

            # objs.insert(0, (obj, normal, bounce_dir))

            # ray.origin = intersection
            # ray.direction = bounce_dir
            for light in lights:
                light_dir = light.position - intersection
                light_dir /= np.linalg.norm(light_dir)
                diffusion = np.dot(light_dir, normal)
                diffusion = np.maximum(0, diffusion)

                light_normal_component = np.dot(light_dir, normal)*normal
                light_perp_component = light_dir - light_normal_component
                light_bounce_dir = light_normal_component - light_perp_component
                specularity = np.dot(-ray.direction, light_bounce_dir)
                specularity /= np.linalg.norm(ray.direction) * \
                    np.linalg.norm(light_bounce_dir)
                specularity = np.maximum(0, specularity)
                ray.color += (
                    obj.material.diffusion*light.color*diffusion +
                    obj.material.specularity*light.color*specularity**obj.material.shininess)

            ray.color += ambient_light
            ray.color *= obj.material.color

    return ray.color


def render(scene, lights, camera, width, height, aa_factor):
    n = aa_factor*width
    m = aa_factor*height

    fov_rads = camera.fov*np.pi/180
    plane_height = 2*camera.focal_length*np.tan(fov_rads/2)
    pixel_size = plane_height/m

    pixels = np.zeros((m, n, 3), np.float32)

    for i in range(m):
        py = -(i - m/2)*pixel_size
        for j in range(n):
            px = (j - n/2)*pixel_size
            near_plane_z = camera.origin[2] - camera.focal_length

            pixel_center = np.array((px, py, near_plane_z))

            ray = Ray(camera.origin, pixel_center - camera.origin, (0, 0, 0))

            pixels[i, j, :] = cast_ray(scene, lights, ray)

    # Shape pixels for display
    pixels = pixels/np.max(pixels)  # HDR
    # pixels = pixels**4              # Gamma correction
    pixels *= 255                   # Convert to 8-bit values

    # Display image
    im = Image.fromarray(pixels.astype(np.uint8), mode="RGB")
    im = im.resize((n//aa_factor, m//aa_factor),
                   resample=Image.Resampling.BILINEAR)
    im.show()


if __name__ == "__main__":
    main()
