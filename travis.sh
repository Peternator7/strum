cd ./strum
cargo build --verbose
cargo test --verbose

cd ..
cd ./strum_macros
cargo test --verbose
cargo test --verbose --features "disable-display"

cd ..
cd ./strum_tests
cargo build --verbose
cargo test --verbose

cd ..
