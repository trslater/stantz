#ifndef GEOMETRY_H
#define GEOMETRY_H

#include "linalg.h"

typedef struct {
    Vector3D origin;
    Vector3D direction;
} Ray;

typedef struct {
    Vector3D normal;
    double offset;
} PlaneGeometry;

typedef struct {
    Vector3D center;
    double radius;
} SphereGeometry;

typedef struct {
    Vector3D origin;
    Vector3D u;
    Vector3D v;
} ParallelogramGeometry;

void sphere_normal( Vector3D, SphereGeometry *, Vector3D );
void parallelogram_normal( Vector3D, ParallelogramGeometry *, Vector3D );

double ray_plane_intersection( Ray *, PlaneGeometry * );
double ray_sphere_intersection( Ray *, SphereGeometry * );
double ray_parallelogram_intersection( Ray *, ParallelogramGeometry * );

#endif
