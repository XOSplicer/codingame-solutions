#include <stdio.h>
#include <string.h>

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
int main()
{
    int N; // the number of temperatures to analyse
    scanf("%d", &N); fgetc(stdin);
    char TEMPS[256]; // the N temperatures expressed as integers ranging from -273 to 5526
    fgets(TEMPS,256,stdin); // the N temperatures expressed as integers ranging from -273 to 5526
    int values[N];
    //printf("Expecting %d integers\n", N);
    //printf("Got string: %s\n", TEMPS);

    char *p;
    //p=TEMPS;
    //printf("*p is %s\n", p);
    //values[0]=strtol(p, &p, 10);
    //printf("1st value is %d\n", values[0]);
    //printf("*p is now %s\n", p);
    int i=0;
    for(p=TEMPS; i<N; i++) {
        values[i]=strtol(p, &p, 10);
	//printf("read %d\n", values[i]);
    }
    //printf("read all\n");
    //for(int i=0; i<N; i++) {
        //printf("values[%d]=%d\n", i, values[i]);
    //}

    if(N==0) {
        printf("0\n");
        return 0;
    }

    int absmin=values[0];
    for(int i=0; i<N; i++) {
        if(abs(values[i])<abs(absmin)) absmin=values[i];
        else if((abs(values[i])==abs(absmin))&&(values[i]>absmin)) absmin=values[i];
    }
    printf("%d\n", absmin);

    return 0;
}
