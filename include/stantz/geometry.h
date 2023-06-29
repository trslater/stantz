#pragma once

#include <Eigen/Dense>

struct Plane
{
    Eigen::Vector3d normal;
    double offset;
};

struct Sphere
{
    Eigen::Vector3d center;
    double radius;
};

struct Parallelogram
{
    Eigen::Vector3d origin;
    Eigen::Vector3d u;
    Eigen::Vector3d v;
};
