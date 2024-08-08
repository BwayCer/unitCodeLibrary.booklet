python OpenCV 開啟鏡頭的特權
=======


## 嘗試

```bash
docker compose up
# 按 "q" 退出
```

等同於

```bash
containerName=local/try/docker_privileged/video_py_cv2

docker image ls --format "{{.Repository}}" | grep $containerName &> /dev/null ||
  docker build -t $containerName -f ./Dockerfile .

docker run --rm -it \
  --privileged \
  --env DISPLAY=$DISPLAY \
  --env XAUTHORITY=/tmp/xauth \
  --volume /tmp/.X11-unix:/tmp/.X11-unix \
  --volume $XAUTHORITY:/tmp/xauth \
  $containerName
```

