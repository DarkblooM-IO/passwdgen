#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int randint(int min, int max) {
  return (rand() % (max - min + 1)) + min;
}

int main(int argc, char **argv) {
  int len;

  switch (argc) {
    case 1:
      len = 16;
      break;
    default:
      len = atoi(argv[1]);
      break;
  }

  srand(time(0));

  for (int i = 0; i < len; i++) {
    printf("%c", randint(33, 126));
  }

  printf("\n");

  return 0;
}
