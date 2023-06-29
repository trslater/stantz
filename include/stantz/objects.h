#pragma once

#include <Eigen/Dense>

#include "geometry.h"
#include "materials.h"

template <typename T>
struct Mesh
{
    T geometry;
    Material material;

    Eigen::Vector3d normal( const Eigen::Vector3d& point );
};
