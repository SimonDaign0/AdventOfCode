// dial with arrow. 0-99
// Rotation is L(dec) or R(inc)
// 11 -R8-> 19 -L19-> 0 -L1-> 99 (count == 1)
// starts at 50
// Goal is, how many times does the dial reaches 0 after a rotation

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
const int DIAL_SIZE = 100;
int parse_nb(char *nb_ptr);

int main() {
  const char *FILE_PATH = "./input.txt";
  FILE *file_ptr = fopen(FILE_PATH, "r");
  if (file_ptr == NULL) {
    printf("Missing input file at path: %s\n", FILE_PATH);
    return -1;
  }

  int amt_at_0 = 0;
  char buf[10];
  int current_index = 50;
  while (fgets(buf, sizeof(buf), file_ptr) != NULL) {
    current_index += parse_nb(buf);
    current_index = ((current_index % DIAL_SIZE) + DIAL_SIZE) % DIAL_SIZE;
    if (current_index == 0) {
      amt_at_0++;
    }
  }
  printf("count: %d\n", amt_at_0);
  fclose(file_ptr);
  return 0;
}

int parse_nb(char *buf) {
  int mod = (buf[0] == 'L') ? -1 : +1;
  int nb = atoi(buf + 1) * mod;
  return nb;
}
