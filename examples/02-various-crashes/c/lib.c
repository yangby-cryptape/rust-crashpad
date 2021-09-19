void cause_segfault (void) {
    *((unsigned int*)0) = 0xDEAD;
}
