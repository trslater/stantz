#pragma once

#include <Eigen/Dense>

struct Camera {
    Eigen::Vector3d origin;
    double fov;
    double focal_length;
};
