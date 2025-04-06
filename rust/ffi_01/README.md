Rust FFI
=======


## 嘗試

```bash
docker compose build

docker run --rm -it local/uc/rust/ffi_01:latest
# docker run --rm -it local/uc/rust/ffi_01:latest /app/myapp --test
# docker run --rm -it local/uc/rust/ffi_01:latest /app/myapp --leak
# docker run --rm -it local/uc/rust/ffi_01:latest /app.rs
# docker run --rm -it local/uc/rust/ffi_01:latest /app.rs --test
# docker run --rm -it local/uc/rust/ffi_01:latest /app.rs --leak

docker rmi local/uc/rust/ffi_01
```


## 簡易的輸出資訊：

LIST:
  - Function

    ```txt
    Lang.c: base_func: This function is called from a C application.
    Lang.rs: base_func: This function is implemented in Rust with "safer-ffi" feature and exposed via a DLL!
    ```

  - bool, 數值, enum

    ```txt
    Lang.c: pure_data: call: true, -8, PureDataArgs { 16, -32, 6.400000, 0 }
    Lang.rs: pure_data: receive call: true, -8, PureDataArgs { 16, -32, 6.4, 0 }
    Lang.rs: pure_data: send response: false, 6303, -1529984654, 0.05945954606105863, 0
    Lang.c: pure_data: result: false, 6303, -1529984654, 0.059460, 0
    ```

  - String

    ```txt
    Lang.c: concat_string: call: Hello, World
    Lang.rs: concat_string: receive call: Hello, World
    Lang.rs: concat_string: send response: Hello World!
    Lang.c: concat_string: result: Hello World!
    Lang.rs: free_string: Hello World!
    ```

  - Array

    ```txt
    Lang.c: concat_array: call: [4, 5, 6], [6, 7, 8, 9]
    Lang.rs: concat_array: receive call: [4, 5, 6], [6, 7, 8, 9]
    Lang.rs: concat_array: send response: [4, 5, 6, 6, 7, 8, 9, 0, 0, -1391134531, 80632145, 1127265303]
    Lang.c: concat_array: result: [4, 5, 6, 6, 7, 8, 9, 0, 0, -1391134531, 80632145, 1127265303]
    Lang.rs: free_array_i32: info: c_slice::Box: (slice_boxed { ptr: NonNullOwned(0x5a16ff8f8d10), len: 12 })
    Lang.rs: free_array_i32: main: [4, 5, 6, 6, 7, 8, 9, 0, 0, -1391134531, 80632145, 1127265303]
    ```

  - Callback

    ```txt
    Lang.c: register_callback: call: callback() (ptr:{0x5a16c658a298})
    Lang.rs: register_callback: receive call: callback() (ptr:0x5a16c658a298)

    Lang.c: trigger_callback: call
    Lang.rs: trigger_callback: receive call: callback() (ptr:0x5a16c658a298)
    Lang.rs: trigger_callback: launch event:
      PureDataArgs { 57740, -67612641, 0.7203636448376959, 2 },
      HUQCIJU,
      [1757178943, -78082151, -40924780, -376975473, -1228522027]
    Lang.c: trigger_callback: receive event (1):
      PureDataArgs { 57740, -67612641, 0.720364, 2 },
      HUQCIJU,
      [1757178943, -78082151, -40924780, -376975473, -1228522027]
    Lang.rs: free_string: HUQCIJU
    Lang.rs: free_array_i32: info: c_slice::Box: (slice_boxed { ptr: NonNullOwned(0x5a16ff8f8cf0), len: 5 })
    Lang.rs: free_array_i32: main: [1757178943, -78082151, -40924780, -376975473, -1228522027]

    Lang.rs: unregister_callback: send response: true
    ```

  - `static`

    ```txt
    Lang.c: static_counter: call
    Lang.rs: static_counter: send response: 3
    Lang.c: static_counter: result: 3
    Lang.rs: static_counter: send response: 4
    Lang.c: static_counter: result: 4
    Lang.rs: static_counter: send response: 5
    Lang.c: static_counter: result: 5

    Lang.c: static_data: call
    Lang.rs: static_data: internal response: init -> 6
    Lang.rs: static_data: internal response: init -> 6 -> 7
    Lang.rs: static_data: internal response: 6 -> 7 -> 8

    Lang.rs: close_static
    ```
