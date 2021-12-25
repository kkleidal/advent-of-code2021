#include <stdint.h>

void shift_in(int input, int* z, int add2) {
  *z = (26 * *z) + (input + add2);
}

void block2(int input, int* z, int add1, int add2) {
  int64_t w, x, y;
  w = input0;
  x = (*z % 26) + add1;
  // We make the change if the first base-26 digit of z + add1 is != input
  x = (x != w);
  // Left shift z (base-26)
  *z /= 26;
  y = 25 * x + 1;
  // Right shift z (base-26)
  *z *= y;
  // Add the input + add2 as the first base-26 digit of z
  y = (w + add2) * x;
  *z += y;

  // If the first base-26 digit of z + add1 is != input:
  //   - replace first base-26 digit of z with input + add2
  // else:
  //   - right shift, removing first base-26 digit of z
}

int run_program(const int64_t* inputs) {
  int64_t w, x, y, z;
  w = x = y = z = 0;
  const int64_t input0 = inputs[0];
  const int64_t input1 = inputs[1];
  const int64_t input2 = inputs[2];
  const int64_t input3 = inputs[3];
  const int64_t input4 = inputs[4];
  const int64_t input5 = inputs[5];
  const int64_t input6 = inputs[6];
  const int64_t input7 = inputs[7];
  const int64_t input8 = inputs[8];
  const int64_t input9 = inputs[9];
  const int64_t input10 = inputs[10];
  const int64_t input11 = inputs[11];
  const int64_t input12 = inputs[12];
  const int64_t input13 = inputs[13];

  // Block 1
  // 3. z = [15]
  shift_in(input0, &z, 12);

  // Block 2
  // 1. z = [15, 8]  (base 26)
  shift_in(input0, &z, 7);

  // Block 3
  // 1. z = [15, 8, 9]
  shift_in(input2, &z, 8);

  // Block 4
  // 6. z = [15, 8, 9, 14]
  shift_in(input3, &z, 8);

  // Block 5
  // 2. z = [15, 8, 9, 14, 17]
  shift_in(input4, &z, 15);

  // Block 6 (first time we divide z)
  // 1. z = [15, 8, 9, 14]
  block2(input5, &z, -16, 12);

  // Block 7
  // 4. z = [15, 8, 9, 14, 12]
  shift_in(input6, &z, 8);

  // Block 8
  // 1. z = [15, 8, 9, 14]
  block2(input7, &z, -11, 13);

  // Block 9
  // 1. z = [15, 8, 9]
  block2(input8, &z, -13, 3);

  // Block 10
  // 1. z = [15, 8, 9, 14]
  shift_in(input9, &z, 13);

  // Block 11
  // 6. z = [15, 8, 9]
  block2(input10, &z, -8, 3);

  // Block 12
  // 8. z = [15, 8]
  block2(input11, &z, -1, 9);

  // Block 13
  // 4. z = [15]
  block2(input12, &z, -4, 4);

  // Block 14
  // 1. z = 0
  block2(input13, &z, -14, 13);

  // 96299896449991
  return z == 0;
}
