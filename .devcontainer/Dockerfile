# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.191.0/containers/rust/.devcontainer/base.Dockerfile
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends 

RUN curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash - \
    && apt-get -y install nodejs --no-install-recommends \
    && npm install -g yarn \
    && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

