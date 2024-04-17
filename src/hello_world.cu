// hello_world.cu
// CUDA: Hello,World
#include <stdio.h>

__global__ void hello_world(void)
{
    printf("GPU: Hello world!\\n");
}

int main(int argc, char **argv)
{
    printf("CPU: Hello world!\\n");
    hello_world<<<1, 10>>>();
    cudaDeviceReset(); // 如果没有这一行，无法从GPU输出 "hello world"
    return 0;
}
