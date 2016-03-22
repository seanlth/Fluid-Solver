
__kernel void vector_add(__global const double *A, __global const double *B, __global double *C) {
	int i = get_global_id(0);
    //int index = i * get_local_size(0) + j;

	C[i] = A[i] + B[i];
}

__kernel void relaxation(global float* new_x, global float* x, global const float* b, unsigned int w, unsigned int h, const float density, const float dt, const float dx, const unsigned int limit) {
    int c = get_global_id(0);
    int r = get_global_id(1);

    local int a = w+2;
    local float scale = (dt / ( density * dx * dx ));

    local float fast_x[1024];

    int local_c = get_local_id(0);
    int local_r = get_local_id(1);

    fast_x[local_r * 32 + local_c] = x[r * ]



    // w = 32;
    // h = 32;

    //for (int l = 0; l < limit; l++) {

        //if ( c - 1 >= 0 && c + 1 < w && r + 1 < h && r - 1 >= 0 ) {


            //     |p3|
            //  ---|--|---
            //  p1 |  | p2
            //  ---|--|---
            //     |p4|


            float alpha = 4.0;

            alpha -= 1.0 * (c == 1);
            alpha -= 1.0 * (c == w);
            alpha -= 1.0 * (r == 1);
            alpha -= 1.0 * (r == h);

            int alpha = (r * a) + c;

            int index1 = alpha - 1;
            int index2 = alpha + 1;
            int index3 = ((r+1) * (a)) + c;
            int index4 = ((r-1) * (a)) + c;


            float p1 = x[index1];
            float p2 = x[index2];
            float p3 = x[index3];
            float p4 = x[index4];


            // float p1 = 0.0;
            // float p2 = 0.0;
            // float p3 = 0.0;
            // float p4 = 0.0;

            // float p1 = (c - 1 >= 0 ? x[(r * w) + c-1] : ( alpha -= 1.0, 0.0 ) ) * (dt / ( density * dx * dx ));
            // float p2 = (c + 1 < w ? x[(r * w) + c+1] : ( alpha -= 1.0, 0.0 ) ) * (dt / ( density * dx * dx ));
            // float p3 = (r + 1 < h ? x[((r+1) * w) + c] : ( alpha -= 1.0, 0.0 ) ) * (dt / ( density * dx * dx ));
            // float p4 = (r - 1 >= 0 ? x[((r-1) * w) + c] : ( alpha -= 1.0, 0.0 ) ) * (dt / ( density * dx * dx ));

            new_x[r * a + c] = ( b[(r-1) * w + (c-1)] + scale*(p1 + p2 + p3 + p4) ) / (alpha * scale);
            //printf("%f ", new_x[r * a + c]);

            //barrier(CLK_GLOBAL_MEM_FENCE);
            //x[r * w + c] = temp[r * w + c];
            //barrier(CLK_GLOBAL_MEM_FENCE);
        //}
    //}
}
