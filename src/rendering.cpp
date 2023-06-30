#include "rendering.h"

// SDL rendering based on https://lazyfoo.net/tutorials/SDL/02_getting_an_image_on_the_screen/index.php
void render( const Scene& scene, const Camera& camera, int num_bounces, const int width, const int height )
{
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

    double fov_rads = camera.fov*3.14159/180;
    double plane_height = 2*camera.focal_length*tan(fov_rads/2);
    double pixel_size = plane_height/height;

    std::vector<std::vector<Eigen::Vector3d>> pixels( height, std::vector<Eigen::Vector3d>( width ) );

    // Loop through pixel squares on image plane
    for( int i = 0; i < height; ++i )
    {
        // Same for every column
        double py = -( i - height/2 )*pixel_size;

        for( int j = 0; j < width; ++j )
        {
            Eigen::Vector3d pixel_center{
                ( j - width/2 )*pixel_size, py,
                camera.origin[2] - camera.focal_length
            };

            Ray ray{
                camera.origin,
                pixel_center - camera.origin,
            };

            pixels[i][j] = cast_ray( ray, scene, num_bounces );
        }
    }

    // Render to screen
    for( int i = 0; i < height; ++i )
    {
        for( int j = 0; j < width; ++j )
        {
            set_pixel( surface, i, j, pixels[i][j].cwiseMin( 1 ) );
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

Eigen::Vector3d cast_ray( const Ray& ray, const Scene& scene, int bounces_left )
{
    double t_min = std::numeric_limits<double>::max();
    const Mesh* mesh;
    bool found = false;
    
    // Object loop
    for( int o = 0; o < scene.objects.size(); ++o )
    {
        double t = scene.objects[o].geometry->intersection( ray );

        if( t >= 0 && t < t_min )
        {
            t_min = t;
            mesh = &scene.objects[o];
            found = true;
        }
    }

    if( !found ) return Eigen::Vector3d{};

    Eigen::Vector3d intersection = ray.at( t_min );
    Eigen::Vector3d normal = mesh->geometry->normal( intersection );

    Eigen::Vector3d pixel;

    // Light loop
    for( int l = 0; l < scene.lights.size(); ++l )
    {
        Eigen::Vector3d light_dir = ( scene.lights[l].position - intersection ).normalized();

        double diffusion = light_dir.dot( normal );
        
        diffusion = std::max( 0., diffusion );

        Eigen::Vector3d camera_dir = -ray.direction.normalized();

        double specularity = ( camera_dir + light_dir ).normalized().dot( normal );
        specularity = std::max( 0., specularity );

        // TODO: Clean this up
        pixel =
            mesh->material.diffusion*scene.lights[l].color*diffusion +
            mesh->material.specularity*scene.lights[l].color*std::pow( specularity, mesh->material.shininess );
    }

    // Apply material colour
    pixel = pixel.cwiseProduct( mesh->material.color );

    return pixel;
}

void set_pixel( SDL_Surface *surface, int i, int j, const Eigen::Vector3d& colour )
{
    Uint32* const pixel = ( Uint32 * )(
        ( Uint8 * ) surface->pixels
        + i*surface->pitch
        + j*surface->format->BytesPerPixel
    );

    *pixel = rgb_to_int( surface->format, colour );
}

Uint32 rgb_to_int( SDL_PixelFormat *format, const Eigen::Vector3d& colour )
{
    return SDL_MapRGB( format, 255*colour[0], 255*colour[1], 255*colour[2] );
}
