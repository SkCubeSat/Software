FROM dockcross/linux-arm64:latest

WORKDIR /root

COPY linux /usr/local/bin/dockross

CMD ["/bin/bash"]
