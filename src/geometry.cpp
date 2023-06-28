#include "geometry.h"

void sphere_normal( Vector3D normal, SphereGeometry *sphere, Vector3D point )
{
    sub_3d( normal, point, sphere->center );
    normalize_3d( normal );
}

void parallelogram_normal( Vector3D normal, ParallelogramGeometry *parallelogram, Vector3D point )
{
    cross( normal, parallelogram->u, parallelogram->v );
    normalize_3d( normal );
}

double ray_plane_intersection( Ray *ray, PlaneGeometry *plane )
{
    double perpendicularness = dot_3d( plane->normal, ray->direction );
        
    if ( perpendicularness >= 0 ) return -1;
    
    return ( plane->offset - dot_3d( plane->normal, ray->origin ) )/perpendicularness;
}

double ray_sphere_intersection( Ray *ray, SphereGeometry *sphere )
{
    // Quadratic coefficients
    double a = dot_3d(ray->direction, ray->direction);
    Vector3D ray_center_diff;
    sub_3d(ray_center_diff, ray->origin, sphere->center);
    double b = 2*dot_3d(ray_center_diff, ray->direction);
    double c = dot_3d(ray_center_diff, ray_center_diff) - sphere->radius*sphere->radius;

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

    return t1 < t2 ? t1 : t2;
}

double ray_parallelogram_intersection( Ray *ray, ParallelogramGeometry *parallelogram )
{
    Matrix3 A;
    matrix_3_from_cols( A, parallelogram->u, parallelogram->v, ray->direction );

    // Does it intersect plane?
    Matrix3 A_inverse;
    if( !inverse_3( A_inverse, A ) ) return -1;

    Vector3D diff;
    sub_3d( diff, ray->origin, parallelogram->origin );

    Vector3D uvt;
    apply_3( uvt, A_inverse, diff );

    uvt[2] *= -1;

    // Is it outside of parallelogram bounds?
    if( uvt[0] < 0 || uvt[0] > 1 || uvt[1] < 0 || uvt[1] > 1 ) return -1;
    
    return uvt[2];
}
