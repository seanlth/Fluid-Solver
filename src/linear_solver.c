#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

double max(double a, double b) {
    return 0.5*(a + b + fabs(a - b));
}

void relaxation_fast_ffi( double* array, double* b, unsigned int w, unsigned int h, double density, double dt, double dx, unsigned int limit )
{
    double* x = array;

    double* new_x = (double*)calloc( (h+2) * (w+2), sizeof(double) );
    double epsilon = 0.01;

    double scale = (dt / ( density * dx * dx ));
    int a = w+2;

    int i = 0;
    for ( ; i < limit; i++ ) {
        double error_delta = 0.0;
        for ( int r = 1; r < h+1; r++ ) {
            for ( int c = 1; c < w+1; c++ ) {

                double alpha = 4.0;

                int cell = (r * a) + c;

                alpha -= 1.0 * (c == 1);
                alpha -= 1.0 * (c == w);
                alpha -= 1.0 * (r == 1);
                alpha -= 1.0 * (r == h);

                int index1 = cell - 1;
                int index2 = cell + 1;
                int index3 = cell + a;
                int index4 = cell - a;

                double p1 = x[index1];
                double p2 = x[index2];
                double p3 = x[index3];
                double p4 = x[index4];

                double new = (  b[(r-1) * w + (c-1)] + scale * ( p1 + p2 + p3 + p4 ) ) / (alpha * scale);

                error_delta = max(error_delta, fabs(new - new_x[cell]));
                new_x[cell] = new;
            }
        }
        double* temp = new_x;
        new_x = x;
        x = temp;

        //if (error_delta < epsilon) { printf("Iteration %d \n", i); break; }
    }

    if ( i != limit ) {
        if ( (i+1) % 2 == 1) {
            memcpy( array, x, sizeof(double) * (h+2) * (w+2) );
            free(x);
        }
        else {
            free(new_x);
        }
    }
    else {

    // after odd number of iteration result is in x, x -> new_x
        if ( limit % 2 == 1 ) {
            memcpy( array, x, sizeof(double) * (h+2) * (w+2) );
            free(x);
        }
        else {
            free(new_x);
        }
    }

    // for (int i = 0; i < h+2; i++) {
    //     for (int j = 0; j < w+2; j++) {
    //         printf("%f ", array[i*(w+2) + j]);
    //     }
    //     printf("\n");
    // }

}


void relaxation_ffi( double* x, double* b, unsigned int w, unsigned int h, double density, double dt, double dx, unsigned int limit )
{
    double* temp = (double*)malloc(sizeof(double) * h * w);
    double epsilon = 0.01;

    for ( int i = 0; i < limit; i++ ) {
        double error_delta = 0.0;
        for ( int r = 0; r < h; r++ ) {
            for ( int c = 0; c < w; c++ ) {

                //     |p3|
                //  ---|--|---
                //  p1 |  | p2
                //  ---|--|---
                //     |p4|


                double alpha = 4.0;

                double p1 = (c - 1 >= 0 ? x[(r * w) + c-1] : ( alpha -= 1.0, 0.0 ) ) * (dt / ( density * dx * dx ));
                double p2 = (c + 1 < w ? x[(r * w) + c+1] : ( alpha -= 1.0, 0.0 ) ) * (dt / ( density * dx * dx ));
                double p3 = (r + 1 < h ? x[((r+1) * w) + c] : ( alpha -= 1.0, 0.0 ) ) * (dt / ( density * dx * dx ));
                double p4 = (r - 1 >= 0 ? x[((r-1) * w) + c] : ( alpha -= 1.0, 0.0 ) ) * (dt / ( density * dx * dx ));

                double new = ( b[r * w + c] + p1 + p2 + p3 + p4 ) / (alpha * (dt / ( density * dx * dx )));

                error_delta = max(error_delta, fabs(new - temp[r * w + c]));
                temp[r * w + c] = new;
            }
        }
        memcpy(x, temp, sizeof(double) * h * w);

        //if (error_delta < epsilon) { break; }
    }

    free(temp);
}
