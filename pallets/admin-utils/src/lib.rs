name: Check Rust

concurrency:
  group: check-rust-${{ github.ref }}
  cancel-in-progress: true

on:
  push:
    branches: [main, devnet-ready, devnet, testnet, finney]

  pull_request:

  ## Allow running workflow manually from the Actions tab
  workflow_dispatch:
    inputs:
      verbose:
        description: "Output more information when triggered manually"
        required: false
        default: ""

env:
  CARGO_TERM_COLOR: always
  VERBOSE: ${{ github.events.input.verbose }}

jobs:
  # runs cargo fmt
  cargo-fmt:
    name: cargo fmt
    runs-on: SubtensorCI
    strategy:
      matrix:
        rust-branch:
          - nightly-2024-03-05
        rust-target:
          - x86_64-unknown-linux-gnu
          # - x86_64-apple-darwin
        os:
          - ubuntu-latest
          # - macos-latest
        include:
          - os: ubuntu-latest
          # - os: macos-latest
    env:
      RELEASE_NAME: development
      # RUSTFLAGS: -A warnings
      RUSTV: ${{ matrix.rust-branch }}
      RUST_BACKTRACE: full
      RUST_BIN_DIR: target/${{ matrix.rust-target }}
      SKIP_WASM_BUILD: 1
      TARGET: ${{ matrix.rust-target }}
    steps:
      - name: Check-out repository under $GITHUB_WORKSPACE
        uses: actions/checkout@v4

      - name: Install dependencies
        run: sudo apt-get update && sudo apt-get install -y build-essential

      - name: Install Rust ${{ matrix.rust-branch }}
        uses: actions-rs/toolchain@v1.0.6
        with:
          toolchain: ${{ matrix.rust-branch }}
          components: rustfmt
          profile: minimal

      - name: cargo fmt
        run: cargo fmt --check --all

  cargo-clippy-default-features:
    name: cargo clippy
    runs-on: SubtensorCI
    strategy:
      matrix:
        rust-branch:
          - stable
        rust-target:
          - x86_64-unknown-linux-gnu
          # - x86_64-apple-darwin
        os:
          - ubuntu-latest
          # - macos-latest
        include:
          - os: ubuntu-latest
          # - os: macos-latest
    env:
      RELEASE_NAME: development
      # RUSTFLAGS: -A warnings
      RUSTV: ${{ matrix.rust-branch }}
      RUST_BACKTRACE: full
      RUST_BIN_DIR: target/${{ matrix.rust-target }}
      SKIP_WASM_BUILD: 1
      TARGET: ${{ matrix.rust-target }}
    steps:
      - name: Check-out repository under $GITHUB_WORKSPACE
        uses: actions/checkout@v4

      - name: Install dependencies
        run: |
          sudo apt-get update &&
          sudo apt-get install -y clang curl libssl-dev llvm libudev-dev protobuf-compiler

      - name: Install Rust ${{ matrix.rust-branch }}
        uses: actions-rs/toolchain@v1.0.6
        with:
          toolchain: ${{ matrix.rust-branch }}
          components: rustfmt, clippy
          profile: minimal

      - name: Utilize Shared Rust Cache
        uses: Swatinem/rust-cache@v2.2.1
        with:
          key: ${{ matrix.os }}-${{ env.RUST_BIN_DIR }}

      - name: cargo clippy --workspace --all-targets -- -D warnings
        run: cargo clippy --workspace --all-targets -- -D warnings

  cargo-clippy-all-features:
    name: cargo clippy --all-features
    runs-on: SubtensorCI
    strategy:
      matrix:
        rust-branch:
          - stable
        rust-target:
          - x86_64-unknown-linux-gnu
          # - x86_64-apple-darwin
        os:
          - ubuntu-latest
          # - macos-latest
        include:
          - os: ubuntu-latest
          # - os: macos-latest
    env:
      RELEASE_NAME: development
      # RUSTFLAGS: -A warnings
      RUSTV: ${{ matrix.rust-branch }}
      RUST_BACKTRACE: full
      RUST_BIN_DIR: target/${{ matrix.rust-target }}
      SKIP_WASM_BUILD: 1
      TARGET: ${{ matrix.rust-target }}
    steps:
      - name: Check-out repository under $GITHUB_WORKSPACE
        uses: actions/checkout@v2

      - name: Install dependencies
        run: |
          sudo apt-get update &&
          sudo apt-get install -y clang curl libssl-dev llvm libudev-dev protobuf-compiler

      - name: Install Rust ${{ matrix.rust-branch }}
        uses: actions-rs/toolchain@v1.0.6
        with:
          toolchain: ${{ matrix.rust-branch }}
          components: rustfmt, clippy
          profile: minimal

      - name: Utilize Shared Rust Cache
        uses: Swatinem/rust-cache@v2.2.1
        with:
          key: ${{ matrix.os }}-${{ env.RUST_BIN_DIR }}

      - name: cargo clippy --workspace --all-targets --all-features -- -D warnings
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings
  # runs cargo test --workspace
  cargo-test:
    name: cargo test
    runs-on: SubtensorCI
    strategy:
      matrix:
        rust-branch:
          - stable
        rust-target:
          - x86_64-unknown-linux-gnu
          # - x86_64-apple-darwin
        os:
          - ubuntu-latest
          # - macos-latest
        include:
          - os: ubuntu-latest
          # - os: macos-latest
    env:
      RELEASE_NAME: development
      # RUSTFLAGS: -A warnings
      RUSTV: ${{ matrix.rust-branch }}
      RUST_BACKTRACE: full
      RUST_BIN_DIR: target/${{ matrix.rust-target }}
      SKIP_WASM_BUILD: 1
      TARGET: ${{ matrix.rust-target }}
    steps:
      - name: Check-out repository under $GITHUB_WORKSPACE
        uses: actions/checkout@v4

      - name: Install dependencies
        run: |
          sudo apt-get update &&
          sudo apt-get install -y clang curl libssl-dev llvm libudev-dev protobuf-compiler

      - name: Install Rust ${{ matrix.rust-branch }}
        uses: actions-rs/toolchain@v1.0.6
        with:
          toolchain: ${{ matrix.rust-branch }}
          components: rustfmt, clippy
          profile: minimal

      - name: Utilize Rust shared cached
        uses: Swatinem/rust-cache@v2.2.1
        with:
          key: ${{ matrix.os }}-${{ env.RUST_BIN_DIR }}

      - name: cargo test --workspace
        run: cargo test --workspace

  # runs cargo test --workspace --features=runtime-benchmarks
  cargo-test-benchmarks:
    name: cargo test w/benchmarks
    runs-on: SubtensorCI
    strategy:
      matrix:
        rust-branch:
          - stable
        rust-target:
          - x86_64-unknown-linux-gnu
          # - x86_64-apple-darwin
        os:
          - ubuntu-latest
          # - macos-latest
        include:
          - os: ubuntu-latest
          # - os: macos-latest
    env:
      RELEASE_NAME: development
      # RUSTFLAGS: -A warnings
      RUSTV: ${{ matrix.rust-branch }}
      RUST_BACKTRACE: full
      RUST_BIN_DIR: target/${{ matrix.rust-target }}
      SKIP_WASM_BUILD: 1
      TARGET: ${{ matrix.rust-target }}
    steps:
      - name: Check-out repository under $GITHUB_WORKSPACE
        uses: actions/checkout@v4

      - name: Install dependencies
        run: |
          sudo apt-get update &&
          sudo apt-get install -y clang curl libssl-dev llvm libudev-dev protobuf-compiler

      - name: Install Rust ${{ matrix.rust-branch }}
        uses: actions-rs/toolchain@v1.0.6
        with:
          toolchain: ${{ matrix.rust-branch }}
          components: rustfmt, clippy
          profile: minimal

      - name: Utilize Rust shared cached
        uses: Swatinem/rust-cache@v2.2.1
        with:
          key: ${{ matrix.os }}-${{ env.RUST_BIN_DIR }}

      - name: cargo test --workspace --features=runtime-benchmarks
        run: cargo test --workspace --features=runtime-benchmarks

  # ensures cargo fix has no trivial changes that can be applied
  cargo-fix:
    name: cargo fix
    runs-on: SubtensorCI
    strategy:
      matrix:
        rust-branch:
          - stable
        rust-target:
          - x86_64-unknown-linux-gnu
          # - x86_64-apple-darwin
        os:
          - ubuntu-latest
          # - macos-latest
        include:
          - os: ubuntu-latest
          # - os: macos-latest
    env:
      RELEASE_NAME: development
      # RUSTFLAGS: -A warnings
      RUSTV: ${{ matrix.rust-branch }}
      RUST_BACKTRACE: full
      RUST_BIN_DIR: target/${{ matrix.rust-target }}
      SKIP_WASM_BUILD: 1
      TARGET: ${{ matrix.rust-target }}
    steps:
      - name: Check-out repository under $GITHUB_WORKSPACE
        uses: actions/checkout@v4

      - name: Install dependencies
        run: |
          sudo apt-get update &&
          sudo apt-get install -y clang curl libssl-dev llvm libudev-dev protobuf-compiler

      - name: Install Rust ${{ matrix.rust-branch }}
        uses: actions-rs/toolchain@v1.0.6
        with:
          toolchain: ${{ matrix.rust-branch }}
          components: rustfmt, clippy
          profile: minimal

      - name: Utilize Rust shared cached
        uses: Swatinem/rust-cache@v2.2.1
        with:
          key: ${{ matrix.os }}-${{ env.RUST_BIN_DIR }}

      - name: cargo fix --workspace
        run: |
          # Run cargo fix on the project
          cargo fix --workspace

          # Check for local git changes
          if ! git diff --exit-code; then
              echo "There are local changes after running 'cargo fix --workspace' ❌"
              exit 1
          else
              echo "No changes detected after running 'cargo fix --workspace' ✅"
          fi

  check-feature-propagation:
    name: zepter run check
    runs-on: SubtensorCI

    steps:
      - name: Install stable Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable

      - name: Install Zepter
        run: cargo install --locked -q zepter && zepter --version

      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Dont clone historic commits.

      - name: Check features
        run: zepter run check

