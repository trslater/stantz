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

    Eigen::Vector3d ray_center_diff = ray.origin() - center;
    double b = 2*ray_center_diff.dot( ray.direction() );
    double c = ray_center_diff.dot( ray_center_diff ) - radius*radius;

    double discriminant = b*b - 4*a*c;

    // Misses
    if ( discriminant < 0 ) return -1;
    
    double first_term = -b/2/a;
    
    // Just grazes surface
    if ( discriminant == 0 ) return first_term;
    
    double second_term = sqrt(discriminant)/2/a;

    // Enter and exit wounds
    double t1 = first_term + second_term;
    double t2 = first_term - second_term;

    // Don't know which one is which, so we return closest
    return t1 < t2 ? t1 : t2;
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
