#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

double max(double a, double b) {
    return 0.5*(a + b + fabs(a - b));
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
