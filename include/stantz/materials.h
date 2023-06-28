#pragma once

#include "linalg.h"

typedef double ColorRGB[3];

struct Material {
    double diffusion;
    double specularity;
    double shininess;
    double reflectance;
    ColorRGB color;
};
