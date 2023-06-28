#ifndef CAMERA_H
#define CAMERA_H

#include "linalg.h"

typedef struct {
    Vector3D origin;
    double fov;
    double focal_length;
} Camera;

#endif
