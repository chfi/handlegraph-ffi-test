ifeq ($(shell uname),Darwin)
	EXT := dylib
else
	EXT := so
endif

all: target/debug/libhandlegraph_ffi.$(EXT)
	g++ src/main.cpp -L ./target/debug/ -lhandlegraph_ffi -o run
	LD_LIBRARY_PATH=./target/debug/ ./run

target/debug/libmath.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf run
