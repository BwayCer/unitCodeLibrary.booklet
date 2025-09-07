<?php
try {
    // 1. FFI 初始化
    // 由 `core.h` C 定義文件載入
    $c_definitions = file_get_contents(__DIR__ . '/core.h');
    $c_dll = __DIR__ . '/core.so';
    $ffi = FFI::cdef($c_definitions, $c_dll);

    // 2. 取得請求的資訊
    $headers = getallheaders();
    $contentType = isset($headers['Content-Type']) ? $headers['Content-Type'] : (isset($headers['content-type']) ? $headers['content-type'] : '');

    $post_data = $_POST;
    if (strpos($contentType, 'application/json') !== false) {
        $json_payload = file_get_contents('php://input');
        $decoded_json = json_decode($json_payload, true);
        if (json_last_error() === JSON_ERROR_NONE) {
            $post_data = $decoded_json;
        }
    }

    $peer_ip = $_SERVER['REMOTE_ADDR'] ?? '';
    $peer_port = (string)($_SERVER['REMOTE_PORT'] ?? '');
    $url = (isset($_SERVER['HTTPS']) && $_SERVER['HTTPS'] === 'on' ? "https" : "http") . "://$_SERVER[HTTP_HOST]$_SERVER[REQUEST_URI]";
    $method = $_SERVER['REQUEST_METHOD'] ?? '';
    $headers_json = json_encode($headers);
    $body_json = json_encode($post_data);
    $files_json = json_encode($_FILES);

    // 3. FFI Call
    $result_cstring = $ffi->process_request(
        $peer_ip,
        $peer_port,
        $url,
        $method,
        $headers_json,
        $body_json,
        $files_json
    );
    // 處理 FFI 字串轉換
    $response_txt = FFI::string($result_cstring);
    $ffi->free_string($result_cstring);

    // 4. 顯示網頁
    $response = json_decode($response_txt);
    http_response_code($response->status);
    header("Content-Type: " . $response->type);
    echo $response->body;
} catch (FFI\Exception $e) {
    // Handle FFI loading errors
    http_response_code(500);
    echo "FFI Error: " . $e->getMessage();
}
?>
