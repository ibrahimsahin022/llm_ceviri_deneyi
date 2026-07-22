# Faz 4 (cok platformlu calistirma) - Linux/LP64 (64-bit long) ortami.
# Windows/LLP64 (32-bit long, MSYS2 gcc + rustup rustc) ile yan yana
# karsilastirma icin; bkz. harness/compare_platforms.py ve MODIFICATIONS.md.
#
# Not: gcc surumu Windows'taki MSYS2 gcc ile bilerek AYNI PIN edilmemistir -
# ikisi zaten farkli derleyici dagitimlaridir (native Linux glibc-gcc vs
# MSYS2/mingw-w64). Asil karsilastirilan degisken gcc surumu degil, `long`
# tipinin ABI genisligidir (LP64=64-bit vs LLP64=32-bit). Gercek surumler
# `gcc --version` / `rustc --version` ile konteyner icinde raporlanir.

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    ca-certificates \
    python3 \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

# rustup ile stabil rustc (Windows tarafinda kullanilan surume en yakin
# stabil kanal; tam surum konteyner icinde `rustc --version` ile dogrulanir)
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain stable --profile minimal

WORKDIR /repo

RUN python3 -m venv /opt/venv
ENV PATH=/opt/venv/bin:$PATH
RUN pip install --no-cache-dir scipy matplotlib

COPY . /repo

CMD ["bash"]
