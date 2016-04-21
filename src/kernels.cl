
// float clamp(float v, float a, float b) {
// 	if ( v < a)  { return a; }
// 	else if ( v > b ) { return b; }
// 	else { return v; }
// }


float catmull_rom_interpolate( float a, float b, float c, float d, float w )  {
    float minimum = min(d, min(c, min(b, a)));
    float maximum = max(d, max(c, max(b, a)));

    float a0 = -0.5*a + 1.5*b - 1.5*c + 0.5*d;
    float a1 = a - 2.5*b + 2.0*c - 0.5*d;
    float a2 = -0.5*a + 0.5*c;
    float a3 = b;

    return min(max(a0*w*w*w + a1*w*w + a2*w + a3, minimum), maximum);
}

float linear_interpolate(float a, float b, float w) {
	return a * w + b * (1.0 - w);
}

float bilinear_interpolate(float x, float y, global float* field, double offset_x, double offset_y, unsigned int rows, unsigned int columns) {
    x = x - offset_x;
	y = y - offset_y;

	x = clamp(x, 0.0f, float(columns - 1.0f));
	y = clamp(y, 0.0f, float(rows - 1.0f));

    int p_x = clamp(int(floor(x)), int(0), int(columns - 1));
    int p1_x = clamp(int(floor(x+1.0)), int(0), int(columns - 1));
    int p_y = clamp(int(floor(y)), int(0), int(rows - 1));
    int p1_y = clamp(int(floor(y+1.0)), int(0), int(rows- 1));

    // printf("%d\n", p_x);
    // printf("%d\n", p1_x);
    // printf("%d\n", p_y);
    // printf("%d\n", p1_y);

    // int p1_x = clamp(int(floor(x+1.0)), int(0), int(columns - 1));
    // int p1_y = clamp(int(floor(y)), int(0), int(rows - 1));

    // int p2_x = clamp(int(floor(x)), int(0), int(columns - 1));
    // int p2_y = clamp(int(floor(y)), int(0), int(rows - 1));
    //
    // int p3_x = clamp(int(floor(x+1.0)), int(0), int(columns - 1));
    // int p3_y = clamp(int(floor(y+1.0)), int(0), int(rows- 1));
    //
    // int p4_x = clamp(int(floor(x)), int(0), int(columns - 1));
    // int p4_y = clamp(int(floor(y+1.0)), int(0), int(rows - 1));

	// weight from 0 to 1 in x and y axis
	float alpha = y - p_y;
	float beta = x - p_x;

	// float p1 = field[p1_y * columns + p1_x];
	// float p2 = field[p2_y * columns + p2_x];
	// float p3 = field[p3_y * columns + p3_x];
	// float p4 = field[p4_y * columns + p4_x];

    float p1 = field[p_y * columns + p1_x];
    float p2 = field[p_y * columns + p_x];
    float p3 = field[p1_y * columns + p1_x];
    float p4 = field[p1_y * columns + p_x];


	// interpolate in x-axis
	float l1 = linear_interpolate(p1, p2, beta);
	float l2 = linear_interpolate(p3, p4, beta);

	// interpolate in y-axis
	return linear_interpolate(l2, l1, alpha);
    //return 1.0;
}
//
// // field must be at least 8x8
// float bicubic_interpolate(float x, float y, global float* field, double offset_x, double offset_y, unsigned int rows, unsigned int columns) {
//
// 	x = x - offset_x;
// 	y = y - offset_y;
//
// 	x = clamp(x, 0.0f, float(columns - 1.0f));
// 	y = clamp(y, 0.0f, float(rows - 1.0f));
//
// 	int x1 = clamp(floor(x-1.0f), 0.0f, columns - 1.0f);
// 	int x2 = clamp(floor(x), 0.0f, columns - 1.0f);
// 	int x3 = clamp(floor(x+1.0f), 0.0f, columns - 1.0f);
// 	int x4 = clamp(floor(x+2.0f), 0.0f, columns - 1.0f);
//
// 	int y1 = clamp(floor(y-1.0f), 0.0f, rows - 1.0f);
// 	int y2 = clamp(floor(y), 0.0f, rows - 1.0f);
// 	int y3 = clamp(floor(y+1.0f), 0.0f, rows - 1.0f);
// 	int y4 = clamp(floor(y+2.0f), 0.0f, rows - 1.0f);
//
// 	int alpha = y - y2;
// 	int beta = x - x2;
//
// 	// interpolate across x-axis
// 	float a = catmull_rom_interpolate(field[y1 * columns + x1], field[y1 * columns + x2], field[y1 * columns + x3], field[y1 * columns + x4], beta );
// 	float b = catmull_rom_interpolate(field[y2 * columns + x1], field[y2 * columns + x2], field[y2 * columns + x3], field[y2 * columns + x4], beta );
// 	float c = catmull_rom_interpolate(field[y3 * columns + x1], field[y3 * columns + x2], field[y3 * columns + x3], field[y3 * columns + x4], beta );
// 	float d = catmull_rom_interpolate(field[y4 * columns + x1], field[y4 * columns + x2], field[y4 * columns + x3], field[y4 * columns + x4], beta );
//
// 	// interpolate across y-axis
// 	return catmull_rom_interpolate(a, b, c, d, alpha );
// }
//
float euler(global float* field, float o, float x, float y, float dt, float dx,
    double offset_x, double offset_y, unsigned int rows, unsigned int columns) {
    return o - bilinear_interpolate(x, y, field, offset_x, offset_y, rows, columns) * dt/dx;
}

__kernel void semi_lagrangian(global const float* field, global float* temp, global const float* u, global const float* v, float dt, float dx,
    float field_offset_x, float field_offset_y, unsigned int field_rows, unsigned int field_columns,
    float u_offset_x, float u_offset_y, unsigned int u_rows, unsigned int u_columns,
    float v_offset_x, float v_offset_y, unsigned int v_rows, unsigned int v_columns) {

    int c = get_global_id(0);
    int r = get_global_id(1);

    float x = c + field_offset_x;
    float y = r + field_offset_y;
    //
    float old_x = euler(u, x, x, y, dt, dx, u_offset_x, u_offset_y, u_rows, u_columns);
    float old_y = euler(v, y, x, y, dt, dx, v_offset_x, v_offset_y, v_rows, v_columns);

    // translate grid(old_x, old_y) -> field_array(i, j)
    temp[r * field_columns + c] = bilinear_interpolate(old_x, old_y, field, field_offset_x, field_offset_y, field_rows, field_columns);
}


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
