#ifndef STANTZ_H
#define STANTZ_H

#include <SDL2/SDL.h>

typedef double ColorRGB[3];

void render( int, int );
void set_pixel( SDL_Surface *, int, int, ColorRGB );
Uint32 rgb_to_int( SDL_PixelFormat *, ColorRGB );

#endif
