#pragma once

#include <Eigen/Dense>

#include "linalg.h"

typedef double ColorRGB[3];

struct Material {
    double diffusion;
    double specularity;
    double shininess;
    double reflectance;
    ColorRGB color;
};
