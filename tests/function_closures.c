int z = 10;

int foo(int a, int b) = {
    if(a == 0 || b == 0) return 0; 
    int x = a + b;
    // The challenge here is that we have no idea where in the stack z is in relation to bar, except at runtime 
    int bar(int c) = {
        return z + x + c; // z comes from the outermost stack frame, x comes from foo's frame, and c is local
    };
    int z = 5; 
    int temp = foo(a - 1, b - 1); // Declares a new bar with a different x but z should still refer to the global z = 10;
    bar(x) + temp + z // z is 5 here
};

foo(1, 2);


