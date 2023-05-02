#include "test.h"

int main( int argc, char const *argv[] )
{
    ObjectList objects;
    init_object_list( &objects, 1 );

    SphereGeometry sphere_geometry = {
        .center = { 0, 0, 0 },
        .radius = 1,
    };
    Material sphere_material = {
        .diffusion = 0.5,
        .specularity = 0.5,
        .shininess = 50,
        .color = { 1, 1, 1 },
    };
    if( !object_list_append( &objects, SPHERE, &sphere_geometry, &sphere_material ) )
    {
        printf( "List full\n" );
    }

    Camera camera = {
        .origin = { 0, 0, 3 },
        .fov = 45,
        .focal_length = 2,
    };

    render( &objects, &camera, 0, atoi( argv[1] ), atoi( argv[2] ) );

    destroy_object_list( &objects );

    return 0;
}
