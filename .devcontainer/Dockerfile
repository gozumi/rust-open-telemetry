# [Choice] Ubuntu version (use ubuntu-22.04 or ubuntu-18.04 on local arm64/Apple Silicon): ubuntu-22.04, ubuntu-20.04, ubuntu-18.04
ARG VARIANT=ubuntu-22.04
FROM mcr.microsoft.com/vscode/devcontainers/base:0-${VARIANT}

# [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

RUN mkdir /home/vscode/git-downloads
RUN git clone https://github.com/marlonrichert/zsh-autocomplete.git /home/vscode/git-downloads/zsh-autocomplete
RUN git clone https://github.com/gozumi/dotfiles.git /home/vscode/git-downloads/dotfiles

COPY library-scripts/git-prompt.sh /home/vscode/git-downloads/git-prompt.sh

RUN rm /home/vscode/.zshrc && ln -s /home/vscode/git-downloads/dotfiles/.zshrc /home/vscode/.zshrc

USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN mkdir /home/vscode/.zfunc
RUN /home/vscode/.cargo/bin/rustup completions zsh > /home/vscode/.zfunc/_rustup 
RUN /home/vscode/.cargo/bin/rustup completions zsh cargo > /home/vscode/.zfunc/_cargo
