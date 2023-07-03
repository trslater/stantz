#pragma once

#include <Eigen/Dense>

class Ray
{
    Eigen::Vector3d _origin;
    Eigen::Vector3d _direction;

public:
    Ray( Eigen::Vector3d origin, Eigen::Vector3d direction );
    Eigen::Vector3d at( double t ) const;
    const Eigen::Vector3d& origin() const;
    const Eigen::Vector3d& direction() const;
};

class Geometry
{
public:
    virtual Eigen::Vector3d normal( const Eigen::Vector3d& point ) = 0;
    virtual double intersection( const Ray& ray ) = 0;
};

class Plane : public Geometry
{
    Eigen::Vector3d _normal;
    double offset;

public:
    Plane( Eigen::Vector3d normal, double offset );
    Eigen::Vector3d normal( const Eigen::Vector3d& point );
    double intersection( const Ray& ray );
};

class Sphere : public Geometry
{
    Eigen::Vector3d center;
    double radius;

public:
    Sphere( Eigen::Vector3d center, double radius );
    Eigen::Vector3d normal( const Eigen::Vector3d& point );
    double intersection( const Ray& ray );
};

class Parallelogram : public Geometry
{
    Eigen::Vector3d origin;
    Eigen::Vector3d u;
    Eigen::Vector3d v;

public:
    Parallelogram( Eigen::Vector3d origin, Eigen::Vector3d u, Eigen::Vector3d v );
    Eigen::Vector3d normal( const Eigen::Vector3d& point );
    double intersection( const Ray& ray );
};
