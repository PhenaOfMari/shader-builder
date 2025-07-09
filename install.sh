SCRIPT_DIR=$(dirname "$(realpath "$0")")
TOOLCHAIN="nightly-2024-11-22"
LIB_DIR="$HOME/.cargo/lib"

echo "Installing toolchain..."
rustup toolchain install $TOOLCHAIN
echo ""

echo "Adding required toolchain components..."
rustup component add --toolchain $TOOLCHAIN rust-src rustc-dev llvm-tools
echo ""

echo "Installing binary..."
cargo "+$TOOLCHAIN" install --path "$SCRIPT_DIR" --locked
mkdir -p "$LIB_DIR"
cp -p "$SCRIPT_DIR/target/release/librustc_codegen_spirv.so" "$LIB_DIR"
