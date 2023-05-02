#include "objects.h"

void init_object_list( ObjectList *objects, int capacity )
{
    objects->items = malloc( capacity*sizeof( Object ) );
    objects->capacity = capacity;
    objects->count = 0;
}

int object_list_append( ObjectList *objects, ObjectType type, void *geometry, Material *material )
{
    if( objects->count >= objects->capacity ) return 0;

    objects->items[objects->count].type = type;
    objects->items[objects->count].geometry = geometry;
    objects->items[objects->count].material = material;

    ++(objects->count);

    return 1;
}

void destroy_object_list( ObjectList *objects )
{
    free( objects->items );
}
