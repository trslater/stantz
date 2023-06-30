#include "rendering.h"

template <>
double Ray::intersection<Plane>( const Plane& plane )
{
    double perpendicularness = plane.normal.dot( direction );
        
    if ( perpendicularness >= 0 ) return -1;
    
    return ( plane.offset - plane.normal.dot( origin ) )/perpendicularness;
}

template <>
double Ray::intersection<Sphere>( const Sphere& sphere )
{
    // Quadratic coefficients
    double a = direction.dot( direction );

    // Ray has ill-defined direction
    if ( a == 0 ) return -1;

    Eigen::Vector3d ray_center_diff = origin - sphere.center;
    double b = 2*ray_center_diff.dot( direction );
    double c = ray_center_diff.dot( ray_center_diff ) - sphere.radius*sphere.radius;

    double discriminant = b*b - 4*a*c;

    // Misses
    if ( discriminant < 0 ) return -1;
    
    double first_term = -b/2/a;
    
    // Just grazes surface
    if ( discriminant == 0 ) return first_term;
    
    double second_term = sqrt(discriminant)/2/a;

    // Enter and exit wounds
    double t1 = first_term + second_term;
    double t2 = first_term - second_term;

    return t1 < t2 ? t1 : t2;
}

template <>
double Ray::intersection<Parallelogram>( const Parallelogram& parallelogram )
{
    Eigen::Matrix3d A {};
    A << parallelogram.u, parallelogram.v, direction;

    printf( "%f %f %f\n", A( 0, 0 ), A( 0, 1 ), A( 0, 2 ) );
    printf( "%f %f %f\n", A( 1, 0 ), A( 1, 1 ), A( 1, 2 ) );
    printf( "%f %f %f\n", A( 2, 0 ), A( 2, 1 ), A( 2, 2 ) );

    Eigen::Vector3d uvt = A.inverse()*( origin - parallelogram.origin );

    uvt[2] *= -1;

    // Is it outside of parallelogram bounds?
    if( uvt[0] < 0 || uvt[0] > 1 || uvt[1] < 0 || uvt[1] > 1 ) return -1;
    
    return uvt[2];
}

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
    return Eigen::Vector3d{ 5, 0, 0, };
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
