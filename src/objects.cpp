#include "objects.h"

template <>
Eigen::Vector3d Mesh<Plane>::normal( const Eigen::Vector3d& point )
{
    return geometry.normal;
}

template <>
Eigen::Vector3d Mesh<Sphere>::normal( const Eigen::Vector3d& point )
{
    return ( point - geometry.center ).normalized();
}

template <>
Eigen::Vector3d Mesh<Parallelogram>::normal( const Eigen::Vector3d& point )
{
    return ( geometry.u.cross(geometry.v) ).normalized();
}
