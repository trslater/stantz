#include "stantz.h"

// SDL rendering based on https://lazyfoo.net/tutorials/SDL/02_getting_an_image_on_the_screen/index.php
void render( int width, int height )
{
    if( SDL_Init( SDL_INIT_VIDEO ) < 0 )
    {
        printf( "SDL could not initialize! SDL_Error: %s\n", SDL_GetError() );
        return;
    }
    
    SDL_Window* window = SDL_CreateWindow(
        "Ray Tracer",
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

    ColorRGB pixels[width*height];

    // Render to screen
    for( int i = 0; i < height; ++i )
    {
        for( int j = 0; j < width; ++j )
        {
            double t = (double)(height - i)/(double)height;
            double s = (double)(width - j)/(double)width;

            pixels[i*width + j][0] = s;
            pixels[i*width + j][1] = t;
            pixels[i*width + j][2] = t;

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
