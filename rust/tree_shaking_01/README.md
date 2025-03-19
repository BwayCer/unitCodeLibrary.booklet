Rust 編譯移除死代碼
=======


## 嘗試

```bash
DOCKER_BUILDKIT=0 docker compose build
docker rmi local/uc/rust/tree_shaking_01
```

其中過程輸出的資訊：

```txt
file: /app/debug/code_3shaking_3print
Hello, world!
2025-03-19T14:40:30.164964Z DEBUG code_3shaking_3print: big text: 1234567890 * 10 * 10382
1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
...
---

file: /app/release/code_3shaking_3print
Hello, world!
---

du -ab /app/*
3983168 /app/debug/code_1common_1nothing
7647176 /app/debug/code_1common_2subscriber
8779824 /app/debug/code_1common_3print
8779872 /app/debug/code_2dead_4notprint
5736600 /app/debug/code_2dead_5notsubscriber_print
8780440 /app/debug/code_3shaking_3print

 435784 /app/release/code_1common_1nothing
 712288 /app/release/code_1common_2subscriber
1775704 /app/release/code_1common_3print
1775720 /app/release/code_2dead_4notprint
1513928 /app/release/code_2dead_5notsubscriber_print
 435784 /app/release/code_3shaking_3print
```
