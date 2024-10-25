set dotenv-load := true

dev:
    @echo "Rustflags=$RUSTFLAGS"
    bun run tauri dev

build:
    @echo "Rustflags=$RUSTFLAGS"
    bun lint && bun tauri build

build-linux:
    @echo "Rustflags=$RUSTFLAGS"
    bun lint && bun tauri build --bundles deb,rpm

build-winx:
    @echo "Rustflags=$RUSTFLAGS"
    bun lint && bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc 

build-both:
    just build-linux
    just build-winx
