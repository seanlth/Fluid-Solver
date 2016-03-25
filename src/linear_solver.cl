
__kernel void vector_add(__global const double *A, __global const double *B, __global double *C) {
	int i = get_global_id(0);
    //int index = i * get_local_size(0) + j;

	C[i] = A[i] + B[i];
}

__kernel void relaxation(global float* new_x, global float* x, global const float* b, unsigned int w, unsigned int h, const float density, const float dt, const float dx) {
    int c = get_global_id(0);
    int r = get_global_id(1);

    local int a;
    a = w+2;
    local float scale;
    scale = (dt / ( density * dx * dx ));

    int cell = (r * a) + c;

    float alpha = 4.0;

    alpha -= 1.0 * (c == 1);
    alpha -= 1.0 * (c == w);
    alpha -= 1.0 * (r == 1);
    alpha -= 1.0 * (r == h);

    int index1 = cell - 1;
    int index2 = cell + 1;
    int index3 = cell + a;
    int index4 = cell - a;

    float p1 = x[index1];
    float p2 = x[index2];
    float p3 = x[index3];
    float p4 = x[index4];

    new_x[cell] = ( b[(r-1) * w + (c-1)] + scale*(p1 + p2 + p3 + p4) ) / (alpha * scale);
}
