# Homebrew installs LLVM in a place that is not visible to ffigen.
# This explicitly specifies the place where the LLVM dylibs are kept.
llvm_path := if os() == "macos" {
    "--llvm-path /opt/homebrew/opt/llvm"
} else {
    ""
}

default: gen lint

gen: _init
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/rust/src/api.rs" "$REPO_DIR/rust/src/json.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated/api.dart" "$REPO_DIR/lib/bridge_generated/json.dart" \
        --rust-output "$REPO_DIR/rust/src/bridge_generated/api.rs" "$REPO_DIR/rust/src/bridge_generated/json.rs" \
        --class-name ApiPlatform ApiJson \
        --c-output "$REPO_DIR/macos/Runner/bridge_generated.h" \
        --skip-add-mod-to-lib
    # Uncomment this line to invoke build_runner as well
    fvm flutter pub run build_runner build --delete-conflicting-outputs

_init:
    #!/usr/bin/env bash
    set -euxo pipefail
    export GENERATED="$PWD/rust/src/bridge_generated"
    if [ ! -e "$GENERATED/api.rs" -a ! -e "$GENERATED/json.rs" ]; then
        touch "$GENERATED/api.rs" "$GENERATED/git.rs" "$GENERATED/ssh.rs"
    fi

device := "hoge"

run:
    fvm flutter run -d {{device}}

lint:
    cd native && cargo fmt
    dart format .

clean:
    flutter clean
    cargo clean

# vim:expandtab:sw=4:ts=4
