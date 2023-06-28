#pragma once

#include <Eigen/Dense>

#include "linalg.h"

struct Ray {
    Vector3D origin;
    Vector3D direction;
};

struct PlaneGeometry {
    Vector3D normal;
    double offset;
};

struct SphereGeometry {
    Vector3D center;
    double radius;
};

struct ParallelogramGeometry {
    Vector3D origin;
    Vector3D u;
    Vector3D v;
};

void sphere_normal( Vector3D, SphereGeometry *, Vector3D );
void parallelogram_normal( Vector3D, ParallelogramGeometry *, Vector3D );

double ray_plane_intersection( Ray *, PlaneGeometry * );
double ray_sphere_intersection( Ray *, SphereGeometry * );
double ray_parallelogram_intersection( Ray *, ParallelogramGeometry * );
