
// float clamp(float v, float a, float b) {
// 	if v < a { return a }
// 	else if v > b { return b }
// 	else { return v }
// }
//
// float runge_kutta_4(float o, float x, float y, global float* field, double offset_x, double offset_y, unsigned int rows, unsigned int columns) {
//
//     float k1 = bicubic_interpolate(x, y, t);
//     let k2 = f(x + (dt / 2.0)*k1, t + dt / 2.0);
//     let k3 = f(x + (dt / 2.0)*k2, t + dt / 2.0);
//     let k4 = f(x + dt*k3, t + dt);
//
//     x + (k1 + 2.0*k2 + 2.0*k3 + k4) * (dt / 6.0)
// }
//
// float catmull_rom_interpolate( float a, float b, float c, float d, float w )  {
//     float minimum = min(d, min(c, min(b, a)));
//     float minimum = max(d, max(c, max(b, a)));
//
//     float a0 = -0.5*a + 1.5*b - 1.5*c + 0.5*d;
//     float a1 = a - 2.5*b + 2.0*c - 0.5*d;
//     float a2 = -0.5*a + 0.5*c;
//     float a3 = b;
//
//     return min(max(a0*w*w*w + a1*w*w + a2*w + a3, minimum), maximum);
// }
//
// // field must be at least 8x8
// float bicubic_interpolate(float x, float y, global float* field, double offset_x, double offset_y, unsigned int rows, unsigned int columns) {
//
// 	x = x - offset_x;
// 	y = y - offset_y;
//
// 	x = clamp(x, 0.0, columns - 1.0);
// 	y = clamp(y, 0.0, rows - 1.0);
//
// 	int x1 = clamp(floor(x-1.0), 0.0, columns - 1.0);
// 	int x2 = clamp(floor(x), 0.0, columns - 1.0);
// 	int x3 = clamp(floor(x+1.0), 0.0, columns - 1.0);
// 	int x4 = clamp(floor(x+2.0), 0.0, columns - 1.0);
//
// 	int y1 = clamp(floor(y-1.0), 0.0, rows - 1.0);
// 	int y2 = clamp(floor(y), 0.0, rows - 1.0);
// 	int y3 = clamp(floor(y+1.0), 0.0, rows - 1.0);
// 	int y4 = clamp(floor(y+2.0), 0.0, rows - 1.0);
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
// 	return catmull_rom_interpolate(a, b, c, d, alpha )
// }
//
// float integrate(float x, float y, global float* field, double offset_x, double offset_y, unsigned int rows, unsigned int columns) {
//     float
// }
//
//
// __kernel void semi_lagrangian(global float* field, global const float* u, global const float* v, float dt, float dx,
//     double field_offset_x, double field_offset_y, unsigned int field_rows, unsigned int field_columns,
//     double u_offset_x, double u_offset_y, unsigned int u_rows, unsigned int u_columns,
//     double v_offset_x, double v_offset_y, unsigned int v_rows, unsigned int v_columns) {
//
//     int c = get_global_id(0);
//     int r = get_global_id(1);
//
//     float x = c + field_offset_x;
//     float y = r + field_offset_y;
//
//     float f1 = -interpolator(x, y, &u)/dx;
//     float f2 = -interpolator(x, y, &v)/dx;
//
//     let old_x = integrator(x, 0.0, &f1, dt);
//     let old_y = integrator(y, 0.0, &f2, dt);
//
//     // translate grid(old_x, old_y) -> field_array(i, j)
//     *temp.at_fast_mut(j, i) = interpolator(old_x, old_y, field);
//
// }


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
