#include "test.h"

int main( int argc, char const *argv[] )
{
    ObjectList objects;
    init_object_list( &objects, 7 );

        // Sphere
    SphereGeometry sphere_geometry = {
        .center = { -0.5, -0.5, 6 },
        .radius = 0.5,
    };
    Material sphere_material = {
        .diffusion = 0.3,
        .specularity = 1,
        .shininess = 50,
        .reflectance = 0.75,
        .color = { 1, 1, 1 },
    };
    if( !object_list_append( &objects, SPHERE, &sphere_geometry, &sphere_material ) )
        printf( "List full\n" );

    // Floor
    PlaneGeometry floor_geometry = {
        .normal = { 0, 1, 0 },
        .offset = -1,
    };
    Material floor_material = {
        .diffusion = 1,
        .specularity = 0,
        .shininess = 0,
        .reflectance = 0.25,
        .color = { .9, .8, .7 },
    };
    if( !object_list_append( &objects, PLANE, &floor_geometry, &floor_material ) )
        printf( "List full\n" );

    // Red wall
    PlaneGeometry red_wall_geometry = {
        .normal = { 1, 0, 0 },
        .offset = -1.5,
    };
    Material red_wall_material = {
        .diffusion = 1,
        .specularity = 0,
        .shininess = 0,
        .reflectance = 0.5,
        .color = { 1, 0, 0 },
    };
    if( !object_list_append( &objects, PLANE, &red_wall_geometry, &red_wall_material ) )
        printf( "List full\n" );

    // Green wall
    PlaneGeometry green_wall_geometry = {
        .normal = { -1, 0, 0 },
        .offset = -1.5,
    };
    Material green_wall_material = {
        .diffusion = 1,
        .specularity = 0,
        .shininess = 0,
        .reflectance = 0.5,
        .color = { 0, 1, 0 },
    };
    if( !object_list_append( &objects, PLANE, &green_wall_geometry, &green_wall_material ) )
        printf( "List full\n" );
    
    // Back wall
    PlaneGeometry back_wall_geometry = {
        .normal = { 0, 0, 1 },
        .offset = 5,
    };
    if( !object_list_append( &objects, PLANE, &back_wall_geometry, &floor_material ) )
        printf( "List full\n" );
    
    // Ceiling
    PlaneGeometry ceiling_geometry = {
        .normal = { 0, -1, 0 },
        .offset = -1.2,
    };
    if( !object_list_append( &objects, PLANE, &ceiling_geometry, &floor_material ) )
        printf( "List full\n" );
    
    // Light
    ParallelogramGeometry light_geometry = {
        .origin = { -0.5, 1, 6.3 },
        .u = { 1, 0, 0 },
        .v = { 0, 0, 1 },
    };
    Material light_material = {
        .diffusion = 0,
        .specularity = 1,
        .shininess = 0,
        .reflectance = 0,
        .color = { 1, 1, 1 },
    };
    if( !object_list_append( &objects, PARALLELOGRAM, &light_geometry, &light_material ) )
        printf( "List full\n" );

    LightList lights;
    init_light_list( &lights, 1 );

    Light light = {
        .position = { 0, 1.17, 6.3 - 0.5 },
        .color = { 1, 1, 1 },
    };

    light_list_append( &lights, &light );

    Camera camera = {
        .origin = { 0, 0, 10 },
        .fov = 45,
        .focal_length = 15,
    };

    render( &objects, &lights, &camera, 10, atoi( argv[1] ), atoi( argv[2] ) );

    destroy_object_list( &objects );
    destroy_light_list( &lights );

    return 0;
}
