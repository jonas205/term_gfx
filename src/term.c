#include <stdint.h>
#include <stdio.h>
#include <termios.h>
#include <unistd.h>

static struct termios old = {0};

void term_disable_stdio_buffer() {
  if (tcgetattr(0, &old) < 0) {
    perror("tcsetattr()");
  }

  old.c_lflag &= ~ICANON;
  old.c_lflag &= ~ECHO;
  old.c_cc[VMIN] = 0;
  old.c_cc[VTIME] = 0;
  if (tcsetattr(0, TCSANOW, &old) < 0) {
    perror("tcsetattr ICANON");
  }
}

void term_reenable_stdio_buffer() {
  old.c_lflag |= ICANON;
  old.c_lflag |= ECHO;
  if (tcsetattr(0, TCSADRAIN, &old) < 0) {
    perror("tcsetattr ~ICANON");
  }
}


uint8_t term_read_char() {
  char buf[1];
  if (read(0, &buf, 1) < 0) {
    perror("read()");
  }
  return buf[0];
}
