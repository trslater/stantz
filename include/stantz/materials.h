#ifndef MATERIALS_H
#define MATERIALS_H

#include <Eigen/Dense>

#include "linalg.h"

typedef double ColorRGB[3];

typedef struct {
    double diffusion;
    double specularity;
    double shininess;
    double reflectance;
    ColorRGB color;
} Material;

#endif
