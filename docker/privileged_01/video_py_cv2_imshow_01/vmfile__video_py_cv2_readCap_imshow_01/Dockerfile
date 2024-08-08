
ARG version=latest

FROM archlinux:$version

ARG mirrorSite=TW
ARG una=bwaycer
ARG uid=1000

# 鏡像站點: 臺灣
RUN curl https://archlinux.org/mirrorlist/?country=$mirrorSite | \
        grep -v 'http:' | sed "s/#Serv/Serv/" > /etc/pacman.d/mirrorlist

# 容器必要更新
RUN pacman-key --init && \
    pacman-key --populate archlinux && \
    pacman -Sy

## 安裝用於測試環境是否正常的工具

# 驗證相機是否正常
#   `mpv av://v4l2:/dev/video0 --profile=low-latency --untimed`
RUN pacman -S --noconfirm mpv

# 驗證 QT 是否正常
RUN pacman -S --noconfirm qt6-base qt6-wayland qtcreator
    # QT: `cv2.imshow()` 的依賴

## 安裝測試的需求環境

RUN pacman -S --noconfirm python mesa
    # mesa: OpenCV 依賴 libGL.so.1 共享庫

RUN useradd -m -u "$uid" -G video "$una"
    # video: /dev/video0

USER "$una"

RUN python -m venv "$HOME/.pyvenv" && \
    source $HOME/.pyvenv/bin/activate && \
    pip install opencv-python

# 載入 Python 環境
ENV VIRTUAL_ENV=/home/$una/.pyvenv
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

COPY ./tryCode_cv2_readCap_imshow.py /tryCode.py

CMD ["python", "/tryCode.py"]

