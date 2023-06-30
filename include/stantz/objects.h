#pragma once

#include <memory>

#include <Eigen/Dense>

#include "geometry.h"
#include "materials.h"

struct Mesh
{
    // TODO: Modernize (unique_ptr or similar)
    Geometry* geometry;
    Material material;
};
