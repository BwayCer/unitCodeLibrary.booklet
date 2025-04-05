#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include "LinkLib.h"

void normal_operation(void);
void test_memory_leak(bool);
char* get_proc_file_name(void);
long read_memory_usage(const char*, long);
void to_usleep(int sec);
void call_base_func(void);
void print_pure_data_args(PureDataArgs_t);
void call_pure_data(void);

int main(int argc, char *argv[]) {
    if (argc > 1 && (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--test-leak") == 0)) {
        test_memory_leak(false);
    } else if (argc > 1 && (strcmp(argv[1], "-l") == 0 || strcmp(argv[1], "--do-leak") == 0)) {
        test_memory_leak(true);
    } else {
        normal_operation();
    }
    return 0;
}

void normal_operation() {
    call_base_func();
    call_pure_data();
}

void test_memory_leak(bool is_leak) {
    char *proc_file_name = get_proc_file_name();

    const int sleep_ms = 5000;
    const int call_interval_sleep_ms = 700;
    const int try_count = 50000;
    const int call_interval_count = 2500;

    int i;
    long first_usage = read_memory_usage(proc_file_name, -1);
    long last_usage = first_usage;

    call_base_func();
    to_usleep(3000);

    for (i = 0; i < try_count; i++) {
        call_pure_data();
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(proc_file_name, last_usage);
            to_usleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(proc_file_name, last_usage);
    to_usleep(sleep_ms);

    for (i = 0; i < try_count; i++) {
        call_base_func();
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(proc_file_name, last_usage);
            to_usleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(proc_file_name, last_usage);

    printf(
        "Memory usage with first: %d KB (diff: %+d KB)\n---\n",
        last_usage,
        last_usage - first_usage
    );

    printf("程序休眠 9 秒後退出...\n");
    fflush(stdout);
    sleep(9);
}

char* get_proc_file_name() {
    // 使用 getpid() 獲取當前程序的 PID
    pid_t pid = getpid();

    // 讀取當前程序的 memory usage，使用 /proc/self/status.
    // 使用靜態變數, 避免返回局部變數地址.
    static char proc_file_name[256];
    snprintf(proc_file_name, sizeof(proc_file_name), "/proc/%d/status", pid);

    return proc_file_name;
}

// 計算消耗的記憶體
long read_memory_usage(const char* file_name, long prev_usage) {
    FILE* file = fopen(file_name, "r");
    if (!file) {
        perror("Unable to open file");
        return -1;
    }

    long value = -1;
    char line[256];
    while (fgets(line, sizeof(line), file)) {
        if (strstr(line, "VmRSS") || strstr(line, "VmSize")) {
            sscanf(line, "%*s %ld", &value);
            break;
        }
    }
    fclose(file);

    if (value == -1) {
        printf("Failed to read memory usage.\n");
        return prev_usage;
    }
    if (prev_usage == -1) {
        printf("---\nMemory usage: %d KB\n---\n", value);
    } else {
        printf("---\nMemory usage: %d KB (diff: %+d KB)\n---\n", value, value - prev_usage);
    }
    return value;
}

void to_usleep(int time_ms) {
    printf("休眠 %d 毫秒...\n", time_ms);
    fflush(stdout);
    usleep(time_ms * 1000);
}

void call_base_func() {
    printf("Lang.c: base_func: This function is called from a C application.\n");
    base_func();
}

void print_pure_data_args(PureDataArgs_t pure_data_args) {
    printf(
        "PureDataArgs { %u, %d, %f, %u }",
        pure_data_args.argu_16,
        pure_data_args.argu_32,
        pure_data_args.argu_64,
        pure_data_args.argu_enum
    );
}

void call_pure_data() {
    bool argu1 = true;
    int8_t argu2 = -8;
    PureDataArgs_t argu3 = {6.4, -32, 16, FRUIT_ENUM_APPLE};
    printf(
        "Lang.c: pure_data: call: %s, %d, ",
        argu1 ? "true" : "false",
        argu2
    );
    print_pure_data_args(argu3);
    printf("\n");

    PureDataResult_t result = pure_data(argu1, argu2, argu3);
    printf(
        "Lang.c: pure_data: result: %s, %u, %d, %f, %u\n",
        result.rtn_yn ? "true" : "false",
        result.rtn_16,
        result.rtn_32,
        result.rtn_64,
        result.rtn_enum
    );
}
