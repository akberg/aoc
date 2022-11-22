int calc(int *input)
{
    int xadd[] = {13, 11, 15, -6, 15, -8, -4, 15, 10, 11, -11,  0, -8, -7};
    int zdiv[] = { 0,  0,  0,  1,  0,  1,  1,  0,  0,  0,   1,  1,  1,  1};
    int zadd[] = { 3, 12,  9, 12,  2,  1,  1, 13,  1,  6,   2, 11, 10,  3};
    int z = 0;

    for (int i = 0; i < 14; i++) {
        int w = input[i];
        int x = (z % 26) + xadd[i];

        if (zdiv[i]) {
            z /= 26;
        }
        
        if (w != x) {
            z = z * 26 + w + zadd[i];
        }
    }
}

/*

w[0] in (0, 9) , z[1] <= 3 + w in (3, 12)
x in (14, 23)
w[1] in (0, 9) , 

w[12] in (1, 9) , z[12] in ()
w[13] in (0, 9) , z[13] in (11, 20)
                  z[14] = 0

*/