docker compose 建立映像文件
=======


## 嘗試

需要先移動到 `./docker-compose.yml` 所在的目錄。

建立 "zh" 映像文件：

```bash
docker compose build zh
```

建立全部映像文件：

```bash
# docker compose build
```

組合操作：

```bash
# 建立全部映像文件 + 執行全部容器
docker compose up

# docker compose down

# docker compose stop

# 移除全部容器
docker compose rm
```

需要手動移除映像文件：

```bash
docker rmi \
  local/uc/docker_compose/build_01/hello_en \
  local/uc/docker_compose/build_01/hello_en:v1 \
  local/uc/docker_compose/build_01/hello_zh
```

