I ended up reverse engineering MONAD in the reverse.c file... It was ugly.

I started out in rust, trying to interpret it, not fast enough. Then I
tried simplifying the AST in rust, not simple enough or fast enough. Then
I tried generating C code and compiling with GCC with optimizations. Not
fast enough.

Finally I looked at the generated C and grouped things into functions and
did a manual greedy optimization to make the shifting end up at 0. MONAD
is basically a series of left shift and conditional right shifts in
base 64. There are 7 left shifts and 7 conditional right shifts. You
need to choose you high order digits to be high/low, but constrained
to the requirement that the condition guarding the right shifts is true
(because you need the right shifts to execute).
