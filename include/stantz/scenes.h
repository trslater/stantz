#pragma once

#include "objects.h"
#include "lights.h"
#include "linalg.h"

struct Scene {
    Object *objects;
    int num_objects;
    Light *lights;
    Vector3D ambient_light;
    int num_lights;
};
