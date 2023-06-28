#pragma once

#include <Eigen/Dense>

#include "linalg.h"

struct Camera {
    Vector3D origin;
    double fov;
    double focal_length;
};
