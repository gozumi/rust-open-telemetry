ARG VARIANT=ubuntu-22.04
FROM mcr.microsoft.com/vscode/devcontainers/base:0-${VARIANT}

RUN apt-get update

RUN apt-get install \
    bat curl gcc git neovim sudo tmux zsh \
    -y 

ARG USERNAME=vscode

# RUN adduser ${USERNAME}

RUN adduser --disabled-password ${USERNAME} --gecos '' ${USERNAME}

RUN echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

RUN chsh -s $(which zsh)

USER ${USERNAME}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -sSf | sh -s -- -y

RUN mkdir /home/${USERNAME}/git-downloads

RUN git clone https://github.com/marlonrichert/zsh-autocomplete.git /home/${USERNAME}/git-downloads/zsh-autocomplete

RUN rm /home/${USERNAME}/.zshrc

RUN git clone https://github.com/gozumi/dotfiles.git /home/${USERNAME}/git-downloads/dotfiles && \
    ln -s /home/${USERNAME}/git-downloads/dotfiles/.zshrc /home/${USERNAME}/.zshrc && \
    ln -s /home/${USERNAME}/git-downloads/dotfiles/git-prompt.sh /home/${USERNAME}/git-prompt.sh && \
    ln -s /home/${USERNAME}/git-downloads/dotfiles/tmux.conf /home/${USERNAME}/tmux.conf

WORKDIR /home/${USERNAME}
