#include "lights.h"

void init_light_list( LightList *lights, int capacity )
{
    lights->items = ( Light ** )malloc( capacity*sizeof( Light * ) );
    lights->capacity = capacity;
    lights->count = 0;
}

int light_list_append( LightList *lights, Light *light )
{
    if( lights->count >= lights->capacity ) return 0;

    lights->items[lights->count] = light;

    ++(lights->count);

    return 1;
}

void destroy_light_list( LightList *lights )
{
    free( lights->items );
}
