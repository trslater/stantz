#include <stdio.h>
#include <string.h>
#include <vector>
#include <memory>

#include <Eigen/Dense>

#include "stantz/cameras.h"
#include "stantz/geometry.h"
#include "stantz/lights.h"
#include "stantz/materials.h"
#include "stantz/objects.h"
#include "stantz/rendering.h"
#include "stantz/scenes.h"

int main( int argc, char const *argv[] )
{
    if( argc < 3 )
    {
        printf( "Usage: test WIDTH HEIGHT\n" );
        return 1;
    }

    Sphere sphere_1( Eigen::Vector3d{ -.5, -.5, 6 }, .5 );
    Sphere sphere_2( Eigen::Vector3d{ 0.5, -0.75, 6 }, .25 );
    Plane floor_plane( Eigen::Vector3d{ 0, 1, 0 }, -1 );
    Plane red_wall_plane( Eigen::Vector3d{ 1, 0, 0 }, -1.5 );
    Plane green_wall_plane( Eigen::Vector3d{ -1, 0, 0 }, -1.5 );
    Plane back_wall_plane( Eigen::Vector3d{ 0, 0, 1 }, 5 );
    Plane front_wall_plane( Eigen::Vector3d{ 0, 0, -1 }, 11 );
    Plane ceiling_plane( Eigen::Vector3d{ 0, -1, 0 }, -1.2 );
    Parallelogram light_fixture_parallelogram(
        Eigen::Vector3d{ -.5, 1, 6.3 },
        Eigen::Vector3d{ 1, 0, 0 },
        Eigen::Vector3d{ 0, 0, 1 }
    );

    Material floor_material{
        1, 0, 0, 0.25,
        Eigen::Vector3d{ .9, .8, .7 },
    };

    Scene scene{
        std::vector<Mesh>{
            Mesh{
                &sphere_1,
                Material{
                    0.3, 1, 50, 0.75,
                    Eigen::Vector3d{ 1, 1, 1 },
                },
            },
            Mesh{
                &sphere_2,
                Material{
                    0.75, 0.25, 10, 0.2,
                    Eigen::Vector3d{ 1, 0, 0 },
                },
            },
            Mesh{ &floor_plane, floor_material },
            Mesh{
                &red_wall_plane,
                Material{
                    1, 0, 0, .5,
                    Eigen::Vector3d{ 1, 0, 0 },
                },
            },
            Mesh{
                &green_wall_plane,
                Material{
                    1, 0, 0, .5,
                    Eigen::Vector3d{ 0, 1, 0 },
                },
            },
            Mesh{ &back_wall_plane, floor_material },
            Mesh{ &front_wall_plane, floor_material },
            Mesh{ &ceiling_plane, floor_material },
            Mesh{
                &light_fixture_parallelogram,
                Material{
                    0, 1, 0, 0,
                    Eigen::Vector3d{ 1, 1, 1 },
                },
            },
        },
        std::vector<Light>{
            Light{
                Eigen::Vector3d{ 0, 1.17, 5.8 },
                Eigen::Vector3d{ 1, 1, 1 },
            },
        },
        Eigen::Vector3d{ .1, .1, .1, },
    };
    
    Camera camera{
        Eigen::Vector3d{ 0, 0, 10 },
        45,
        15,
    };

    render( scene, camera, 10, atoi( argv[1] ), atoi( argv[2] ) );

    return 0;
}
