#include <stdio.h>
#include <stdlib.h>
#include <fenv.h>
#include <math.h>

int main()
{
#pragma STDC FENV_ACCESS ON
    fesetround(FE_DOWNWARD);
    printf("%d\n", fegetround());
    puts("rounding down: ");
    printf("           pi = %.22f\n", acosf(-1));
    printf("strtof(\"1.1\") = %.22f\n", strtof("1.1", NULL));
    printf("    rint(2.1) = %.22f\n\n", rintf(2.1));
    fesetround(FE_UPWARD);
    printf("%d\n", fegetround());
    puts("rounding up: ");
    printf("           pi = %.22f\n", acosf(-1));
    printf("strtof(\"1.1\") = %.22f\n", strtof("1.1", NULL));
    printf("    rint(2.1) = %.22f\n", rintf(2.1));
}