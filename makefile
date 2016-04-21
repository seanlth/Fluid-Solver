
build:
	if [ ! -d "./target" ]; then mkdir -p target/release/deps; mkdir -p ./target/debug/deps; fi
	clang -Ofast -c src/linear_solver.c
	ar rvs libsolver.a linear_solver.o
	rm linear_solver.o
	cp libsolver.a target/debug/deps/
	mv libsolver.a target/release/deps/
	cargo build --release

clean:
	cargo clean

run:
	cargo run --release

bench:
	cargo bench

example:
	cargo test --release
