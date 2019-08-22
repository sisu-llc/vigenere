/*
  Transcribed from Coursera Cryptography course, Lesson 1,
  taught by Prof. Jonathan Katz.

  This is not my code and I claim no ownership...I did change
  it to just use stdin/stdout instead of hardcoded local files,
  i.e. using fdopen(3) instead of fopen(3).

 */

#include <stdio.h>
#define KEY_LENGTH 2

int main() {
	unsigned char ch;
	FILE *fpIn = stdin, *fpOut = stdout;

	int i;
	unsigned char key[KEY_LENGTH] = {0xA8, 0x38};

	i = 0;
	while (fscanf(fpIn, "%c", &ch) != EOF) {
		if (ch != '\n') {
			fprintf(fpOut, "%02X", ch ^ key[i % KEY_LENGTH]);
			i++;
		}
	}

	return 0;
}
