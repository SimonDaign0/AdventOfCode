#include <stdio.h>
#include <string.h>
#define BANK_SIZE 12
unsigned long solve_line(char line[]);
void strip_end(char line[]);
unsigned long custom_atoi(int bk[], char line[]);

int main() {
  const char *FILE_PATH = "./input.txt";
  FILE *file_ptr = fopen(FILE_PATH, "r");
  if (file_ptr == NULL) {
    printf("Missing input file at path: %s\n", FILE_PATH);
    return -1;
  }

  char line_buf[200];
  unsigned long total = 0;
  while (fgets(line_buf, sizeof(line_buf), file_ptr) != NULL) {
    strip_end(line_buf);
    unsigned long joltage = solve_line(line_buf);
    total += joltage;
  }
  printf("sum joltage : %lu\n", total);
  fclose(file_ptr);
  return 0;
}

unsigned long solve_line(char line[]) {
  int linelen = strlen(line);
  int bk[BANK_SIZE];
  int start = 0;
  int amt_chosen = 0;
  while (amt_chosen < BANK_SIZE) {
    int remaining = BANK_SIZE - amt_chosen;
    // last possible index that can fit all remaining batteries
    int last_start = linelen - remaining;
    int best_pos = start;
    for (int i = start; i <= last_start; i++) {
      if (line[i] > line[best_pos]) {
        best_pos = i;
      }
    }
    start = best_pos + 1;
    bk[amt_chosen++] = best_pos;
  }
  unsigned long joltage = custom_atoi(bk, line);
  return joltage;
}

void strip_end(char line[]) {
  int len = strlen(line);
  while (line[len - 1] == '\n' || line[len - 1] == '\r' ||
         line[len - 1] == ' ') {
    line[len - 1] = '\0';
    len--;
  }
}

unsigned long custom_atoi(int bk[], char line[]) {
  unsigned long total = 0;
  for (int i = 0; i < BANK_SIZE; i++) {
    total = total * 10 + (line[bk[i]] - '0');
  }
  return total;
}
