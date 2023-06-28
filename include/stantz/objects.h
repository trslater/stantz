#pragma once

#include <math.h>
#include <stdlib.h>

#include <Eigen/Dense>

#include "linalg.h"
#include "materials.h"

enum ObjectType {
    PLANE,
    PARALLELOGRAM,
    SPHERE,
};

struct Object {
    ObjectType type;
    void *geometry;
    Material *material;
};

struct ObjectList{
    Object *items;
    int capacity;
    int count;
};

void init_object_list( ObjectList *, int );
int object_list_append( ObjectList *, ObjectType, void *, Material * );
void destroy_object_list( ObjectList * );
