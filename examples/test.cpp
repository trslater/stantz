#include <Eigen/Dense>

#include <stdio.h>
#include <string.h>
#include <variant>
#include <vector>

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

    Material floor_material{
        1, 0, 0, 0.25,
        Eigen::Vector3d{ .9, .8, .7 },
    };

    Scene scene{
        ObjectList{
            Mesh<Sphere>{
                Sphere{
                    Eigen::Vector3d{ -0.5, -0.5, 6 },
                    .5,
                },
                Material{
                    0.3, 1, 50, 0.75,
                    Eigen::Vector3d{ 1, 1, 1 },
                },
            },
            Mesh<Sphere>{
                Sphere{
                    Eigen::Vector3d{ 0.5, -0.75, 6 },
                    .25,
                },
                Material{
                    0.75, 0.25, 10, 0.2,
                    Eigen::Vector3d{ 1, 0, 0 },
                },
            },
            
            // Floor
            Mesh<Plane>{
                Plane{
                    Eigen::Vector3d{ 0, 1, 0 },
                    -1,
                },
                floor_material,
            },
            
            // Red wall
            Mesh<Plane>{
                Plane{
                    Eigen::Vector3d{ 1, 0, 0 },
                    -1.5,
                },
                Material{
                    1, 0, 0, .5,
                    Eigen::Vector3d{ 1, 0, 0 },
                },
            },
            
            // Green wall
            Mesh<Plane>{
                Plane{
                    Eigen::Vector3d{ -1, 0, 0 },
                    -1.5,
                },
                Material{
                    1, 0, 0, .5,
                    Eigen::Vector3d{ 0, 1, 0 },
                },
            },
            
            // Back wall
            Mesh<Plane>{
                Plane{
                    Eigen::Vector3d{ 0, 0, 1 },
                    5,
                },
                floor_material,
            },
            
            // Front wall
            Mesh<Plane>{
                Plane{
                    Eigen::Vector3d{ 0, 0, -1 },
                    11,
                },
                floor_material,
            },
            
            // Ceiling
            Mesh<Plane>{
                Plane{
                    Eigen::Vector3d{ 0, -1, 0 },
                    -1.2,
                },
                floor_material,
            },
            
            // Light fixture
            Mesh<Parallelogram>{
                Parallelogram{
                    Eigen::Vector3d{ -.5, 1, 6.3 },
                    Eigen::Vector3d{ 1, 0, 0 },
                    Eigen::Vector3d{ 0, 0, 1 },
                },
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
