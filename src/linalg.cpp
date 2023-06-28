#include "linalg.h"

double dot_3d( Vector3D u, Vector3D v )
{
    return u[0]*v[0] + u[1]*v[1] + u[2]*v[2];
}

void cross( Vector3D w, Vector3D u, Vector3D v )
{
    w[0] = u[1]*v[2] - u[2]*v[1];
    w[1] = u[2]*v[0] - u[0]*v[2];
    w[2] = u[0]*v[1] - u[1]*v[0];
}

double det_2( Matrix2 A )
{
    return A[0][0]*A[1][1] - A[0][1]*A[1][0];
}

double det_3( Matrix3 A )
{
    Matrix2 M_00, M_01, M_02;

    minor_3( M_00, A, 0, 0 );
    minor_3( M_01, A, 0, 1 );
    minor_3( M_02, A, 0, 2 );

    return A[0][0]*det_2( M_00 ) - A[0][1]*det_2( M_01 ) + A[0][2]*det_2( M_02 );
}

void minor_3( Matrix2 M, Matrix3 A, int i, int j )
{
    int rows_skipped = 0, cols_skipped = 0;
    
    for( int k = 0; k < 3; ++k )        // Rows
    {
        if( k == i )
        {
            ++rows_skipped;
            continue;
        }
        for( int l = 0; l < 3; ++l )    // Columns
        {
            if( l == j )
            {
                ++cols_skipped;
                continue;
            }
            
            M[k - rows_skipped][l - cols_skipped] = A[k][l];
        }

        cols_skipped = 0;
    }
}

int inverse_3( Matrix3 A_inverse, Matrix3 A )
{
    double A_det = det_3( A );

    if( A_det == 0 ) return 0;

    Matrix2 M_00, M_01, M_02,
            M_10, M_11, M_12,
            M_20, M_21, M_22;

    minor_3( M_00, A, 0, 0 );
    minor_3( M_01, A, 0, 1 );
    minor_3( M_02, A, 0, 2 );
    minor_3( M_10, A, 1, 0 );
    minor_3( M_11, A, 1, 1 );
    minor_3( M_12, A, 1, 2 );
    minor_3( M_20, A, 2, 0 );
    minor_3( M_21, A, 2, 1 );
    minor_3( M_22, A, 2, 2 );
    
    A_inverse[0][0] = det_2( M_00 )/A_det;
    A_inverse[1][0] = -det_2( M_01 )/A_det;
    A_inverse[2][0] = det_2( M_02 )/A_det;
    A_inverse[0][1] = -det_2( M_10 )/A_det;
    A_inverse[1][1] = det_2( M_11 )/A_det;
    A_inverse[2][1] = -det_2( M_12 )/A_det;
    A_inverse[0][2] = det_2( M_20 )/A_det;
    A_inverse[1][2] = -det_2( M_21 )/A_det;
    A_inverse[2][2] = det_2( M_22 )/A_det;

    return 1;
}

void neg_3d( Vector3D v )
{
    v[0] = -v[0];
    v[1] = -v[1];
    v[2] = -v[2];
}

void add_3d( Vector3D w, Vector3D u, Vector3D v )
{
    w[0] = u[0] + v[0];
    w[1] = u[1] + v[1];
    w[2] = u[2] + v[2];
}

void sub_3d( Vector3D w, Vector3D u, Vector3D v )
{
    w[0] = u[0] - v[0];
    w[1] = u[1] - v[1];
    w[2] = u[2] - v[2];
}

void copy_3d( Vector3D w, Vector3D v )
{
    w[0] = v[0];
    w[1] = v[1];
    w[2] = v[2];
}

void scale_3d( Vector3D v, double c )
{
    v[0] *= c;
    v[1] *= c;
    v[2] *= c;
}

double norm_3d( Vector3D v )
{
    return sqrt( v[0]*v[0] + v[1]*v[1] + v[2]*v[2] );
}

void normalize_3d( Vector3D v )
{
    scale_3d( v, 1/norm_3d(v) );
}

void project_3d( Vector3D w, Vector3D u, Vector3D v )
{
    scale_3d( w, dot_3d( u, v )/norm_3d( u ) );
}

// Normal must be unit vector
void reflect_3d( Vector3D out, Vector3D normal, Vector3D point, Vector3D in )
{
    double normal_component_mag = dot_3d( in, normal );

    // Light behind object
    if( normal_component_mag > 1 ) return;

    Vector3D double_normal_component;
    copy_3d( double_normal_component, normal );
    scale_3d( double_normal_component, 2*normal_component_mag );

    sub_3d( out, in, double_normal_component );
}

void matrix_3_from_cols( Matrix3 A, Vector3D u, Vector3D v, Vector3D w )
{
    A[0][0] = u[0];
    A[1][0] = u[1];
    A[2][0] = u[2];
    A[0][1] = v[0];
    A[1][1] = v[1];
    A[2][1] = v[2];
    A[0][2] = w[0];
    A[1][2] = w[1];
    A[2][2] = w[2];
}

void apply_3( Vector3D b, Matrix3 A, Vector3D x )
{
    b[0] = A[0][0]*x[0] + A[0][1]*x[1] + A[0][2]*x[2];
    b[1] = A[1][0]*x[0] + A[1][1]*x[1] + A[1][2]*x[2];
    b[2] = A[2][0]*x[0] + A[2][1]*x[1] + A[2][2]*x[2];
}

void print_matrix_2( Matrix2 A )
{
    print_vector_2d( A[0] );
    print_vector_2d( A[1] );
}

void print_matrix_3( Matrix3 A )
{
    print_vector_3d( A[0] );
    print_vector_3d( A[1] );
    print_vector_3d( A[2] );
}

void print_vector_2d( Vector2D v )
{
    printf( "%f %f\n", v[0], v[1] );
}

void print_vector_3d( Vector3D v )
{
    printf( "%f %f %f\n", v[0], v[1], v[2] );
}
