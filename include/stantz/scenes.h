#pragma once

#include <variant>
#include <vector>

#include <Eigen/Dense>

#include "objects.h"
#include "lights.h"

struct Scene {
    // TODO: Find better way to do this
    std::vector<Mesh> objects;
    std::vector<Light> lights;
    Eigen::Vector3d ambient_light;
};
