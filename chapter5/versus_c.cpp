#include <cassert>
int x = 10;
int &r = x ;
assert(r==10);
r = 20;