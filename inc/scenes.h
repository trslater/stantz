#ifndef SCENES_H
#define SCENES_H

#include "objects.h"
#include "lights.h"
#include "linalg.h"

typedef struct {
    Object *objects;
    int num_objects;
    Light *lights;
    Vector3D ambient_light;
    int num_lights;
} Scene;

#endif
