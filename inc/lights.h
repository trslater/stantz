#ifndef LIGHTS_H
#define LIGHTS_H

#include <stdlib.h>

#include "linalg.h"
#include "materials.h"

typedef struct {
    Vector3D position;
    ColorRGB color;
} Light;

typedef struct {
    Light **items;
    int capacity;
    int count;
} LightList;

void init_light_list( LightList *, int );
int light_list_append( LightList *, Light * );
void destroy_light_list( LightList * );

#endif