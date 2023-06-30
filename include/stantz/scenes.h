#pragma once

#include <variant>
#include <vector>

#include <Eigen/Dense>

#include "objects.h"
#include "lights.h"

using ObjectList = std::vector<
    std::variant<
        Mesh<Plane>,
        Mesh<Sphere>,
        Mesh<Parallelogram>
    >
>;

struct Scene {
    // TODO: Find better way to do this
    ObjectList objects;
    std::vector<Light> lights;
    Eigen::Vector3d ambient_light;
};
