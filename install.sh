echo "Building release executable."
cargo build --release

echo
echo "--> Installing cli under 'menu'"
sudo mv target/release/menu-cli /bin/menu