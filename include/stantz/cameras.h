#pragma once

#include "linalg.h"

struct Camera {
    Vector3D origin;
    double fov;
    double focal_length;
};
