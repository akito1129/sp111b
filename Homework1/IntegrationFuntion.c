#include <stdio.h>

double integrate(double (*f)(double), double a, double b) {
    double sum = 0.0, step = 0.0001;
    for (double i = a; i < b; i += step) {
        sum += f(i) * step;
    }
    return sum;
}

double square(double x) {
    return x*x;
}

int main(void) {
    printf("integrate(square, 0.0, 2.0)=%f\n", integrate(square, 0.0, 2.0));
}