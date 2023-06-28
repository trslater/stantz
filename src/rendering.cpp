#include "rendering.h"

// SDL rendering based on https://lazyfoo.net/tutorials/SDL/02_getting_an_image_on_the_screen/index.php
void render( ObjectList *objects, LightList *lights, Camera *camera, int num_bounces, int width, int height )
{
    // printf
    if( SDL_Init( SDL_INIT_VIDEO ) < 0 )
    {
        printf( "SDL could not initialize! SDL_Error: %s\n", SDL_GetError() );
        return;
    }
    
    SDL_Window* window = SDL_CreateWindow(
        "Stantz",
        SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
        width, height,
        SDL_WINDOW_SHOWN
    );
    if( window == NULL )
    {
        printf( "Window could not be created! SDL_Error: %s\n", SDL_GetError() );
        return;
    }
    
    SDL_Surface *surface = SDL_GetWindowSurface( window );

    double fov_rads = camera->fov*3.14159/180;
    double plane_height = 2*camera->focal_length*tan(fov_rads/2);
    double pixel_size = plane_height/height;

    ColorRGB pixels[width*height];

    // Init pixels to black
    for( int i = 0; i < height; ++i )
    {
        for( int j = 0; j < width; ++j )
        {
            pixels[i*width + j][0] = 0;
            pixels[i*width + j][1] = 0;
            pixels[i*width + j][2] = 0;
        }
    }

    // Loop through pixel squares on image plane
    for( int i = 0; i < height; ++i )
    {
        double py = -(i - height/2)*pixel_size;

        for( int j = 0; j < width; ++j )
        {
            double px = (j - width/2)*pixel_size;
            double near_plane_z = camera->origin[2] - camera->focal_length;

            Vector3D pixel_center = { px, py, near_plane_z };

            Ray ray;
            copy_3d( ray.origin, camera->origin );
            copy_3d( ray.direction, pixel_center );
            sub_3d( ray.direction, ray.direction, camera->origin );

            cast_ray( pixels[i*width + j], &ray, objects, lights, num_bounces );
        }
    }

    // Find max norm
    double mag, max_mag = 1;
    for( int i = 0; i < height; ++i )
    {
        for( int j = 0; j < width; ++j )
        {
            mag = norm_3d( pixels[i*width + j] );

            if( mag > max_mag )
            {
                max_mag = mag;
            }
        }
    }

    // Render to screen
    for( int i = 0; i < height; ++i )
    {
        for( int j = 0; j < width; ++j )
        {
            // Normalize to [0, 1]
            pixels[i*width + j][0] /= max_mag;
            pixels[i*width + j][1] /= max_mag;
            pixels[i*width + j][2] /= max_mag;

            set_pixel( surface, i, j, pixels[i*width + j] );
        }
    }
    SDL_UpdateWindowSurface( window );

    // Hack to get window to stay up
    SDL_Event e;
    int quit = 0;
    while( quit == 0 )
    {
        while( SDL_PollEvent( &e ) )
        {
            if( e.type == SDL_QUIT ) quit = 1;
        }
    }

    SDL_DestroyWindow( window );
    SDL_Quit();
}

void cast_ray( Vector3D pixel, Ray *ray, ObjectList *objects, LightList *lights, int bounces_left )
{
    double t, min_t = DBL_MAX;
    Vector3D normal, intersection;
    Material *material;
    int found = 0;
    
    // Object loop
    for( int o = 0; o < objects->count; ++o )
    {
        if( objects->items[o].type == PLANE)
        {
            PlaneGeometry *plane_geometry = ( PlaneGeometry * )objects->items[o].geometry;
            t = ray_plane_intersection( ray, plane_geometry );
            if( t >= 0 && t < min_t )
            {
                min_t = t;
                ray_point( intersection, ray, t );
                copy_3d( normal, plane_geometry->normal );
                material = objects->items[o].material;
                found = 1;
            }
        }
        else if( objects->items[o].type == PARALLELOGRAM )
        {
            ParallelogramGeometry *parallelogram_geometry = ( ParallelogramGeometry * )objects->items[o].geometry;
            t = ray_parallelogram_intersection( ray, parallelogram_geometry );
            if( t >= 0 && t < min_t )
            {
                min_t = t;
                ray_point( intersection, ray, t );
                parallelogram_normal( normal, parallelogram_geometry, intersection );
                material = objects->items[o].material;
                found = 1;
            }
        }
        else if( objects->items[o].type == SPHERE )
        {
            SphereGeometry *sphere_geometry = ( SphereGeometry * )objects->items[o].geometry;
            t = ray_sphere_intersection( ray, sphere_geometry );
            if( t >= 0 && t < min_t )
            {
                min_t = t;
                ray_point( intersection, ray, t );
                sphere_normal( normal, sphere_geometry, intersection );
                material = objects->items[o].material;
                found = 1;
            }
        }
    }

    if( !found ) return;

    // Light loop
    for( int l = 0; l < lights->count; ++l )
    {
        Vector3D light_dir;
        sub_3d( light_dir, lights->items[l]->position, intersection );
        normalize_3d( light_dir );

        double diffusion = dot_3d( light_dir, normal );
        
        diffusion = fmax( 0, diffusion );

        Vector3D camera_dir;
        copy_3d( camera_dir, ray->direction );
        normalize_3d( camera_dir );
        neg_3d( camera_dir );

        Vector3D light_camera_midpoint;
        add_3d( light_camera_midpoint, camera_dir, light_dir );
        normalize_3d( light_camera_midpoint );
        
        double specularity = dot_3d( light_camera_midpoint, normal );
        specularity = fmax( 0, specularity );

        pixel[0] += 
            material->diffusion*lights->items[l]->color[0]*diffusion +
            material->specularity*lights->items[l]->color[0]*pow( specularity, material->shininess );
        pixel[1] +=
            material->diffusion*lights->items[l]->color[1]*diffusion +
            material->specularity*lights->items[l]->color[1]*pow( specularity, material->shininess );
        pixel[2] +=
            material->diffusion*lights->items[l]->color[2]*diffusion +
            material->specularity*lights->items[l]->color[2]*pow( specularity, material->shininess );
    }

    // Apply material colour
    pixel[0] *= material->color[0];
    pixel[1] *= material->color[1];
    pixel[2] *= material->color[2];

    Ray reflected_ray;
    reflect_3d( reflected_ray.direction, normal, intersection, ray->direction );
    copy_3d( reflected_ray.origin, intersection );
    
    // copy_3d( reflected_ray.origin, reflected_ray.direction );
    // scale_3d( reflected_ray.origin, 0.00001 );
    // add_3d( reflected_ray.origin, reflected_ray.origin, intersection );

    if( bounces_left <= 0 || material->reflectance == 0 ) return;

    Vector3D temp_pixel = { 0, 0, 0 };

    cast_ray( temp_pixel, &reflected_ray, objects, lights, bounces_left - 1 );

    pixel[0] += material->reflectance*temp_pixel[0];
    pixel[1] += material->reflectance*temp_pixel[1];
    pixel[2] += material->reflectance*temp_pixel[2];
}

void ray_point( Vector3D intersection, Ray *ray, double t )
{
    Vector3D distance;
    copy_3d( distance, ray->direction );
    scale_3d( distance, t );
    add_3d( intersection, ray->origin, distance );
}

void set_pixel( SDL_Surface *surface, int i, int j, ColorRGB c )
{
    Uint32 * const pixel = ( Uint32 * )(
        ( Uint8 * ) surface->pixels
        + i*surface->pitch
        + j*surface->format->BytesPerPixel
    );

    *pixel = rgb_to_int( surface->format, c );
}

Uint32 rgb_to_int( SDL_PixelFormat *format, ColorRGB c )
{
    return SDL_MapRGB( format, 255*c[0], 255*c[1], 255*c[2] );
}
