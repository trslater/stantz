#pragma once

#include <stdlib.h>

#include <Eigen/Dense>

#include "linalg.h"
#include "materials.h"

struct Light {
    Vector3D position;
    ColorRGB color;
};

struct LightList {
    Light **items;
    int capacity;
    int count;
};

void init_light_list( LightList *, int );
int light_list_append( LightList *, Light * );
void destroy_light_list( LightList * );
