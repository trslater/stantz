#pragma once

#include <Eigen/Dense>

struct Material {
    double diffusion;
    double specularity;
    double shininess;
    double reflectance;
    Eigen::Vector3d color;
};
