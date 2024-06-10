#include <stdio.h>
#include <stdlib.h>
#include <sys/utsname.h>
#include <string.h>

int main()
{
    struct utsname unameData;
    uname(&unameData);

    // 打印内核版本
    printf("Kernel version: %s\n", unameData.release);

    // 打印CPU架构
    printf("Architecture: %s\n", unameData.machine);

    // 打印GCC版本（通过预定义宏）
    printf("GCC version: %s\n", __VERSION__);

    // 打印操作系统
    printf("Operating System: %s\n", unameData.sysname);

    // 由于设备制造商和基础库不是直接可通过标准API获取的，这两项信息较为特殊，通常需要特定环境下的工具或查询硬件信息的库。
    // 这里我们假定是在Linux环境下，使用常规方法获取可能的信息。

    // 打印设备制造商（示例，实际使用时需要根据具体环境调整）
    FILE *manufacturer = fopen("/sys/class/dmi/id/sys_vendor", "r");
    if (manufacturer)
    {
        char vendor[100];
        fgets(vendor, sizeof(vendor), manufacturer);
        printf("Device Manufacturer: %s", vendor); // fgets保留换行符
        fclose(manufacturer);
    }
    else
    {
        printf("Device Manufacturer: Unknown\n");
    }

    // 打印C标准库版本（GNU C Library）
    printf("GNU libc version: %s\n", gnu_get_libc_version());

    return 0;
}