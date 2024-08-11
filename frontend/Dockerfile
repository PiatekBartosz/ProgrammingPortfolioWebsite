FROM debian:bookworm-slim

ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=$USER_UID

WORKDIR /workspaces
COPY ./ .

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt update \
    && apt install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# Install dependencies
RUN apt-get update \
    && apt-get install -y \
    build-essential \
    pkg-config \
    libssh-dev

# ********************************************************
# * Anything else you want to do like clean up goes here *
# ********************************************************

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME

# Install rust as 'dev' user
RUN sudo apt install -y curl \
    && curl https://sh.rustup.rs -sSf | bash -s -- -y \
    && echo '. ${HOME}/.cargo/env' >> ${HOME}/.bashrc

# # Yew and dependencies   
RUN . ${HOME}/.cargo/env \
    && rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk

RUN sudo apt install -y npm
