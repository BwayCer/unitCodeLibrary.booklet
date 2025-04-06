#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include "LinkLib.h"

void normal_operation(void);
void test_memory_leak(bool, bool);
char* get_proc_file_name(void);
long read_memory_usage(bool, const char*, long, long);
void to_sleep(int sec);
void call_base_func(void);
void print_pure_data_args(PureDataArgs_t);
void call_pure_data(bool);
void call_concat_string(bool, bool);
void print_array_ref(slice_ref_int32_t slice);
void print_array_boxed(slice_boxed_int32_t slice);
void call_concat_array(bool, bool);
void call_register_callback(bool, bool, uint8_t);
void call_trigger_callback(bool);
void call_static_counter(bool);
void call_static_data(bool);

int main(int argc, char *argv[]) {
    bool is_normal = true;
    if (argc > 1) {
        bool is_trace = strcmp(argv[1], "--trace") == 0;
        if (strcmp(argv[1], "-l") == 0 || strcmp(argv[1], "--leak") == 0) {
            is_normal = false;
            test_memory_leak(true, true);
        } else if (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--test") == 0) {
            is_normal = false;
            test_memory_leak(is_trace, false);
        }
    }
    if (is_normal) {
        normal_operation();
    }
    return 0;
}

void normal_operation() {
    bool is_trace = true;
    bool is_leak = false;

    call_base_func();
    call_pure_data(is_trace);
    call_concat_string(is_trace, is_leak);
    call_concat_array(is_trace, is_leak);

    call_register_callback(is_trace, is_leak, 1);
    call_trigger_callback(is_trace);
    unregister_callback(is_trace);
    call_register_callback(is_trace, is_leak, 2);
    call_trigger_callback(is_trace);
    unregister_callback(is_trace);

    for (int i = 0; i < 3; i++) {
        call_static_counter(is_trace);
        call_static_data(is_trace);
        close_static(is_trace);
    }
}

void test_memory_leak(bool is_trace, bool is_leak) {
    char *proc_file_name = get_proc_file_name();

    const int sleep_ms = 5000;
    const int call_interval_sleep_ms = 700;
    const int try_count = 50000;
    const int call_interval_count = 2500;

    long first_usage = read_memory_usage(true, proc_file_name, 0, 0);
    long last_usage = first_usage;

    call_base_func();
    printf("---\n");
    last_usage = read_memory_usage(false, proc_file_name, last_usage, first_usage);
    to_sleep(3000);

    printf("\n---\ntest: call_pure_data\n");
    for (int i = 0; i < try_count; i++) {
        call_pure_data(is_trace);
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
            to_sleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    to_sleep(sleep_ms);

    printf("\n---\ntest: call_concat_string\n");
    for (int i = 0; i < try_count; i++) {
        call_concat_string(is_trace, is_leak);
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
            to_sleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    to_sleep(sleep_ms);

    printf("\n---\ntest: call_concat_array\n");
    for (int i = 0; i < try_count; i++) {
        call_concat_array(is_trace, is_leak);
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
            to_sleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    to_sleep(sleep_ms);

    printf("\n---\ntest: call_trigger_callback\n");
    call_register_callback(is_trace, is_leak, 1);
    for (int i = 0; i < try_count; i++) {
        call_trigger_callback(is_trace);
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
            to_sleep(call_interval_sleep_ms);
        }
    }
    unregister_callback(is_trace);
    last_usage = read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    to_sleep(sleep_ms);

    printf("\n---\ntest: call_register_callback, unregister_callback\n");
    for (int i = 0; i < try_count; i++) {
        call_register_callback(is_trace, is_leak, (i % 2) + 1);
        call_trigger_callback(is_trace);
        unregister_callback(is_trace);
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
            to_sleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    to_sleep(sleep_ms);

    printf("\n---\ntest: call_static_counter\n");
    for (int i = 0; i < try_count; i++) {
        call_static_counter(is_trace);
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
            to_sleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    to_sleep(sleep_ms);

    printf("\n---\ntest: call_static_data\n");
    for (int i = 0; i < try_count; i++) {
        call_static_data(is_trace);
        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
            to_sleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    to_sleep(sleep_ms);

    printf("\n---\ntest: close_static\n");
    for (int i = 0; i < try_count; i++) {
        for (int o = 0; o < 11; o++) {
            call_static_counter(is_trace);
            call_static_data(is_trace);
        }
        if (!is_leak) {
            close_static(is_trace);
        }

        if (i % call_interval_count == 0 && i != 0) {
            read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
            to_sleep(call_interval_sleep_ms);
        }
    }
    last_usage = read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    to_sleep(sleep_ms);

    printf("\n---\ntest: wait 9 sec\n");
    for (int i = 0; i < 9; i++) {
        to_sleep(1000);
        read_memory_usage(is_trace, proc_file_name, last_usage, first_usage);
    }
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
long read_memory_usage(bool is_trace, const char* file_name, long prev_usage, long first_usage) {
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
    if (is_trace) {
        printf("---\n");
    } else {
        printf("\r");
    }
    if (prev_usage == 0) {
        printf("Memory usage: %d KB\n", value);
    } else {
        printf(
            "Memory usage: %d KB (diff: %+d KB) (diff: %+d KB with first)\n",
            value,
            value - prev_usage,
            value - first_usage
        );
    }
    if (is_trace) {
        printf("---\n");
    }
    return value;
}

void to_sleep(int time_ms) {
    printf("休眠 %d 毫秒...", time_ms);
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

void call_pure_data(bool is_trace) {
    bool argu1 = true;
    int8_t argu2 = -8;
    PureDataArgs_t argu3 = {6.4, -32, 16, FRUIT_ENUM_APPLE};
    if (is_trace) {
        printf(
            "Lang.c: pure_data: call: %s, %d, ",
            argu1 ? "true" : "false",
            argu2
        );
        print_pure_data_args(argu3);
        printf("\n");
    }

    PureDataResult_t result = pure_data(is_trace, argu1, argu2, argu3);
    if (is_trace) {
        printf(
            "Lang.c: pure_data: result: %s, %u, %d, %f, %u\n",
            result.rtn_yn ? "true" : "false",
            result.rtn_16,
            result.rtn_32,
            result.rtn_64,
            result.rtn_enum
        );
    }
}

void call_concat_string(bool is_trace, bool is_leak) {
    const char *argu1 = "Hello";
    const char *argu2 = "World";
    if (is_trace) {
        printf("Lang.c: concat_string: call: %s, %s\n", argu1, argu2);
    }

    char *result = concat_string(is_trace, argu1, argu2);
    if (result) {
        if (is_trace) {
            printf("Lang.c: concat_string: result: %s\n", result);
        }

        if (!is_leak) {
            // free(result);
            // or
            free_string(is_trace, result);
        }

        // or
        // // 把字串複製為本地變數
        // char local_copy[strlen(result) + 1];
        // strcpy(local_copy, result);
        //
        // if (!is_leak) {
        //     free_string(is_trace, result);
        // }
        //
        // if (is_trace) {
        //     printf("Lang.c: concat_string: result: %s\n", local_copy);
        // }
    } else {
        printf("Lang.c: concat_string: result: (null ptr)\n");
    }
}

void print_array_ref(slice_ref_int32_t slice) {
    if (slice.len == 0) {
        printf("[]");
        return;
    }

    printf("[%d", slice.ptr[0]);
    for (size_t i = 1; i < slice.len; ++i) {
        printf(", %d", slice.ptr[i]);
    }
    printf("]");
}

void print_array_boxed(slice_boxed_int32_t slice) {
    if (slice.len == 0) {
        printf("[]");
        return;
    }

    printf("[%d", slice.ptr[0]);
    for (size_t i = 1; i < slice.len; ++i) {
        printf(", %d", slice.ptr[i]);
    }
    printf("]");
}


void call_concat_array(bool is_trace, bool is_leak) {
    int32_t arr1[] = {4, 5, 6};
    slice_ref_int32_t argu1 = {arr1, sizeof(arr1) / sizeof(arr1[0])};
    int32_t arr2[] = {6, 7, 8, 9};
    slice_ref_int32_t argu2 = {arr2, sizeof(arr2) / sizeof(arr2[0])};
    if (is_trace) {
        printf("Lang.c: concat_array: call: ");
        print_array_ref(argu1);
        printf(", ");
        print_array_ref(argu2);
        printf("\n");
    }

    slice_boxed_int32_t result = concat_array(is_trace, argu1, argu2);

    if (is_trace) {
        printf("Lang.c: concat_array: result: ");
        print_array_boxed(result);
        printf("\n");
    }

    if (!is_leak) {
        // free(result);
        // or
        free_array_i32(is_trace, result);
    }
}

void launch_event_base(
    uint8_t index, bool is_trace, bool is_leak, PureDataArgs_t pure_data_args,
    char * char_boxed, slice_boxed_int32_t array_boxed) {

    if (is_trace) {
        printf("Lang.c: trigger_callback: receive event (%u):\n  ", index);
        print_pure_data_args(pure_data_args);
        printf(",");
        if (char_boxed) {
            printf("\n  %s,", char_boxed);
        } else {
            printf("\n  (null ptr),", char_boxed);
            printf("Lang.c: concat_string: result: \n");
        }
        printf("\n  ");
        print_array_boxed(array_boxed);
        printf("\n");
    }


    if (!is_leak) {
        // free(char_boxed);
        // free(array_boxed);
        // or
        free_string(is_trace, char_boxed);
        free_array_i32(is_trace, array_boxed);
    }
}

void launch_event_01(bool is_trace, bool is_leak, PureDataArgs_t argu1, char * argu2, slice_boxed_int32_t argu3) {
    launch_event_base(1, is_trace, is_leak, argu1, argu2, argu3);
}

void launch_event_02(bool is_trace, bool is_leak, PureDataArgs_t argu1, char * argu2, slice_boxed_int32_t argu3) {
    launch_event_base(2, is_trace, is_leak, argu1, argu2, argu3);
}

typedef void (*LaunchEvent)(bool, bool, PureDataArgs_t, char *, slice_boxed_int32_t);

void call_register_callback(bool is_trace, bool is_leak, uint8_t launch_event_index) {
    LaunchEvent callback = (launch_event_index == 2) ? launch_event_02 : launch_event_01;
    if (is_trace) {
        printf("Lang.c: register_callback: call: callback() (ptr:{%p})\n", callback);
    }
    register_callback(is_trace, is_leak, callback);
}

void call_trigger_callback(bool is_trace) {
    if (is_trace) {
        printf("Lang.c: trigger_callback: call\n");
    }
    int i;
    for (i = 0; i < 3; i++) {
        trigger_callback(is_trace);
    }
}

void call_static_counter(bool is_trace) {
    if (is_trace) {
        printf("Lang.c: static_counter: call\n");
    }
    int i;
    for (i = 0; i < 3; i++) {
        uint32_t result = static_counter(is_trace);
        if (is_trace) {
            printf("Lang.c: static_counter: result: %u\n", result);
        }
    }
}

void call_static_data(bool is_trace) {
    if (is_trace) {
        printf("Lang.c: static_data: call\n");
    }
    int i;
    for (i = 0; i < 3; i++) {
        static_data(is_trace);
    }
}
