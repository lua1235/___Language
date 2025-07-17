int pow2divinlined(int x) = if(x % 2 == 0) {
    x /= 2;
    1 + if(x % 2 == 0) {
        x /= 2;
        1 + if(x % 2 == 0) {
            x /= 2;
            1 + if(x % 2 == 0) {
                1
            } else {
                0
            }
        } else {
            0
        }
    } else {
        0
    }
} else {
    0
}

int pow2div(int x) = if(x % 2 == 0) {
    x /= 2;
    1 + log2(x)
} else {
    0
};
