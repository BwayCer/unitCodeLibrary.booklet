以 FFI 操控 PHP 頁面
=======


## 嘗試

```bash
docker compose build
docker compose up
```

送出 `multipart/form-data` 資料：

```bash
mkdir /tmp/uv_php_ffi_server_01
echo -e "test file 01\nabcd" > /tmp/uv_php_ffi_server_01/file1.txt
echo -e "test file 02\nefgh" > /tmp/uv_php_ffi_server_01/file2.txt

curl -X POST http://localhost:8080/user/alice/file \
     -F "username=Alice" \
     -F "email=alice@example.com" \
     -F "file1=@/tmp/uv_php_ffi_server_01/file1.txt" \
     -F "file2=@/tmp/uv_php_ffi_server_01/file2.txt"
```
