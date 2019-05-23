cd ./strum
cargo build --verbose
cargo test --verbose

cd ..
cd ./strum_tests
cargo build --verbose
cargo test --verbose

cd ..
cd ./strum_tests_rename
cargo build --verbose
cargo test --verbose

cd ..
cd ./rust_2018_tests/
cargo build --verbose
cargo test --verbose

cd ..
