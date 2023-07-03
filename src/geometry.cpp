#include "geometry.h"

Ray::Ray( Eigen::Vector3d origin, Eigen::Vector3d direction )
: _origin( origin ), _direction( direction.normalized() )
{}

Eigen::Vector3d Ray::at( double t ) const
{
    return _origin + _direction*t;
}

const Eigen::Vector3d& Ray::origin() const
{
    return _origin;
}

const Eigen::Vector3d& Ray::direction() const
{
    return _direction;
}

Plane::Plane( Eigen::Vector3d normal, double offset )
    : _normal( normal ), offset( offset )
{}

Eigen::Vector3d Plane::normal( const Eigen::Vector3d& point )
{
    return _normal;
}

double Plane::intersection( const Ray& ray )
{
    double perpendicularness = _normal.dot( ray.direction() );
        
    if ( perpendicularness >= 0 ) return -1;
    
    return ( offset - _normal.dot( ray.origin() ) )/perpendicularness;
}

Sphere::Sphere( Eigen::Vector3d center, double radius )
    : center( center ), radius( radius )
{}

Eigen::Vector3d Sphere::normal( const Eigen::Vector3d& point )
{
    return ( point - center ).normalized();
}

double Sphere::intersection( const Ray& ray )
{
    // Quadratic coefficients
    double a = ray.direction().dot( ray.direction() );

    // Ray has ill-defined direction
    if ( a == 0 ) return -1;

    Eigen::Vector3d u = ray.origin() - center;
    double b = ray.direction().dot( u );
    double c = u.dot( u ) - radius*radius;

    double discriminant = b*b - c;

    // Misses
    if ( discriminant < 0 ) return -1;
    
    // Just grazes surface (i.e., tangent to surface)
    if ( discriminant == 0 ) return -b;
    
    double second_term = sqrt(discriminant);

    // Since second_term is always positive, (-) root is always smaller
    return -b - second_term;
}

Parallelogram::Parallelogram( Eigen::Vector3d origin, Eigen::Vector3d u, Eigen::Vector3d v )
    : origin( origin ), u( u ), v( v )
{}

Eigen::Vector3d Parallelogram::normal( const Eigen::Vector3d& point )
{
    return ( u.cross( v ) ).normalized();
}

double Parallelogram::intersection( const Ray& ray )
{
    Eigen::Matrix3d A {};
    A << u, v, ray.direction();

    Eigen::Vector3d uvt = A.inverse()*( ray.origin() - origin );

    uvt[2] *= -1;

    // Is it outside of parallelogram bounds?
    if( uvt[0] < 0 || uvt[0] > 1 || uvt[1] < 0 || uvt[1] > 1 ) return -1;
    
    return uvt[2];
}
