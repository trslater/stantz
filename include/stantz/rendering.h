#pragma once

#include <algorithm>
#include <array>
#include <cmath>

#include <Eigen/Dense>
#include <SDL2/SDL.h>

#include "cameras.h"
#include "geometry.h"
#include "lights.h"
#include "materials.h"
#include "objects.h"
#include "scenes.h"

void render( const Scene&, const Camera&, int, int, int );
Eigen::Vector3d cast_ray( const Ray&, const Scene&, int );
void ray_point( Eigen::Vector3d, Ray *, double );
void set_pixel( SDL_Surface *, int, int, const Eigen::Vector3d& );
Uint32 rgb_to_int( SDL_PixelFormat *, const Eigen::Vector3d& );
