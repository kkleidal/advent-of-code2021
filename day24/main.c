#include <stdio.h>
#include <stdint.h>

int run_program(const int64_t* inputs);

int main(int argc, void* argv) {
  printf("Hello world!\n");

  int64_t input[14] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

  for (uint64_t x = 99999999999999ull; x > 9999999999999ull; --x) {
    if (x % 10000000 == 0) {
      printf("%f%% done\n", (double) (99999999999999ull - x) / (double) (99999999999999ull - 9999999999999ull) * 100);
    }
    input[0] = (int64_t) ((x / 10000000000000ull) % 10ull);
    input[1] = (int64_t) ((x / 1000000000000ull) % 10ull);
    input[2] = (int64_t) ((x / 100000000000ull) % 10ull);
    input[3] = (int64_t) ((x / 10000000000ull) % 10ull);
    input[4] = (int64_t) ((x / 1000000000ull) % 10ull);
    input[5] = (int64_t) ((x / 100000000ull) % 10ull);
    input[6] = (int64_t) ((x / 10000000ull) % 10ull);
    input[7] = (int64_t) ((x / 1000000ull) % 10ull);
    input[8] = (int64_t) ((x / 100000ull) % 10ull);
    input[9] = (int64_t) ((x / 10000ull) % 10ull);
    input[10] = (int64_t) ((x / 1000ull) % 10ull);
    input[11] = (int64_t) ((x / 100ull) % 10ull);
    input[12] = (int64_t) ((x / 10ull) % 10ull);
    input[13] = (int64_t) (x % 10ull);
    int zero = 0;
    for (int i = 0; i < 14; ++i) {
      if (input[i] == 0) {
        zero = 1;
      }
    }
    if (zero) {
      continue;
    }
    if (run_program((const int64_t*) input)) {
      printf("Solution: %zu\n", x);
      return 0;
    }
  }
  printf("No solution found.\n");
  return 1;
}
