#!/bin/bash
/usr/bin/arm-linux-gnueabihf-gcc -static test.s -o test.bin && ./test.bin