#ifndef LINALG_H
#define LINALG_H

#include <stdio.h>
#include <math.h>

typedef double Vector2D[2];
typedef double Vector3D[3];

typedef Vector2D Matrix2[2];
typedef Vector3D Matrix3[3];


// VECTORS
// ----------------------------------------------------------------------------

double dot_3d( Vector3D, Vector3D );
void cross( Vector3D, Vector3D, Vector3D );

void copy_3d( Vector3D, Vector3D );
void add_3d( Vector3D, Vector3D, Vector3D );
void sub_3d( Vector3D, Vector3D, Vector3D );
void scale_3d( Vector3D, double );
void neg_3d( Vector3D );
void normalize_3d( Vector3D );
double norm_3d( Vector3D );

void project_3d( Vector3D, Vector3D, Vector3D );
void reflect_3d( Vector3D, Vector3D, Vector3D, Vector3D );

void print_vector_2d( Vector2D );
void print_vector_3d( Vector3D );


// MATRICES
// ----------------------------------------------------------------------------

void matrix_3_from_cols( Matrix3, Vector3D, Vector3D, Vector3D );

double det_2( Matrix2 );
double det_3( Matrix3 );

void minor_3( Matrix2, Matrix3, int, int );
int inverse_3( Matrix3, Matrix3 );

void apply_3( Vector3D, Matrix3, Vector3D );

void print_matrix_2( Matrix2 );
void print_matrix_3( Matrix3 );

#endif
