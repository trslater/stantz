#ifndef STANTZ_H
#define STANTZ_H

#include <SDL2/SDL.h>

#include "cameras.h"
#include "geometry.h"
#include "lights.h"
#include "linalg.h"
#include "materials.h"
#include "objects.h"

void render( ObjectList *, LightList *, Camera *, int, int, int );
void cast_ray( Vector3D, Ray *, ObjectList *, LightList *, int );
void ray_point( Vector3D, Ray *, double );
void set_pixel( SDL_Surface *, int, int, ColorRGB );
Uint32 rgb_to_int( SDL_PixelFormat *, ColorRGB );

#endif
