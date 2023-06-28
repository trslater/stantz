#ifndef OBJECTS_H
#define OBJECTS_H

#include <math.h>
#include <stdlib.h>

#include <Eigen/Dense>

#include "linalg.h"
#include "materials.h"

typedef enum {
    PLANE,
    PARALLELOGRAM,
    SPHERE,
} ObjectType;

typedef struct {
    ObjectType type;
    void *geometry;
    Material *material;
} Object;

typedef struct {
    Object *items;
    int capacity;
    int count;
} ObjectList;

void init_object_list( ObjectList *, int );
int object_list_append( ObjectList *, ObjectType, void *, Material * );
void destroy_object_list( ObjectList * );

#endif
