#include <stdio.h>
#include <stdlib.h>
const int DIAL_SIZE = 100;
int parse_nb(char *nb_ptr);

int main() {
  const char *FILE_PATH = "./input.txt";
  FILE *file_ptr = fopen(FILE_PATH, "r");
  if (file_ptr == NULL) {
    printf("Missing input file at path: %s\n", FILE_PATH);
    return -1;
  }

  int ticks_at_0 = 0;
  char buf[10];
  int curr = 50;
  while (fgets(buf, sizeof(buf), file_ptr) != NULL) {
    int n = parse_nb(buf); // rotation amount +/-
    if (n >= DIAL_SIZE - curr) {
      ticks_at_0 += (curr + n) / DIAL_SIZE;
    } else if (-n >= curr) {
      ticks_at_0 += (-n - curr + DIAL_SIZE) / 100;
      if (curr == 0) {
        ticks_at_0--;
      }
    }
    curr = (curr + n) % 100;

    if (curr < 0) {
      curr += 100;
    }
  }
  printf("count: %d\n", ticks_at_0);
  fclose(file_ptr);
  return 0;
}

int parse_nb(char *buf) {
  int mod = (buf[0] == 'L') ? -1 : +1;
  int nb = atoi(buf + 1) * mod;
  return nb;
}
