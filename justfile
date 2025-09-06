build:
    mkdir -p build
    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin  
    cargo build --release --target x86_64-pc-windows-gnu
    cargo build --release --target x86_64-unknown-linux-gnu
    cp target/x86_64-apple-darwin/release/lumi build/lumi-mac-x64
    cp target/aarch64-apple-darwin/release/lumi build/lumi-mac-arm64
    cp target/x86_64-pc-windows-gnu/release/lumi.exe build/lumi-windows-x64.exe
    cp target/x86_64-unknown-linux-gnu/release/lumi build/lumi-linux-x64

clean:
    cargo clean
    rm -rf build/