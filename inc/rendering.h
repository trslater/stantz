#ifndef STANTZ_H
#define STANTZ_H

#include <SDL2/SDL.h>

#include "cameras.h"
#include "geometry.h"
#include "linalg.h"
#include "materials.h"
#include "objects.h"

void render( ObjectList *, Camera *, int, int, int );
void cast_ray( Vector3D, ObjectList *, Ray *, int );
void ray_point( Vector3D, Ray *, double );
void set_pixel( SDL_Surface *, int, int, ColorRGB );
Uint32 rgb_to_int( SDL_PixelFormat *, ColorRGB );

#endif
