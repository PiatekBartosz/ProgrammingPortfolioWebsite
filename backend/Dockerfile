FROM debian:bookworm-slim

ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=$USER_UID

ARG BASE_DIR=/workspaces

WORKDIR ${BASE_DIR}
COPY . .

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME


RUN apt-get update --fix-missing && \
    apt-get install -y g++ \
                       cmake \
                       pkg-config \
                       meson \
                       python3-setuptools \
                       libssl-dev \
                       curl \
                       unzip \
                       git \
                       zlib1g-dev \
                       nlohmann-json3-dev \
                       libboost-dev \
                       libboost-iostreams-dev \
                       libboost-system-dev \
                       libboost-serialization-dev && \
    apt-get -qq -y autoremove && \
    apt-get autoclean && \
    rm -rf /var/lib/apt/lists/* /var/log/dpkg.log

# Install pistache
RUN git clone https://github.com/pistacheio/pistache.git && \
    cd ${BASE_DIR}/pistache && \
    meson setup build \
      --buildtype=release \
      -DPISTACHE_USE_SSL=true \
      -DPISTACHE_BUILD_EXAMPLES=false \
      -DPISTACHE_BUILD_TESTS=false \
      -DPISTACHE_BUILD_DOCS=false \
      --prefix=/usr && \
    meson compile -C build && \
    meson install -C build

# ********************************************************
# * Anything else you want to do like clean up goes here *
# ********************************************************

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME
