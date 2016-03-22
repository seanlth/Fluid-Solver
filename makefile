
build:
	clang -Ofast -c src/linear_solver.c
	ar rvs libsolver.a linear_solver.o
	rm linear_solver.o
	cp libsolver.a target/debug/deps/
	mv libsolver.a target/release/deps/
	cargo build

clean:
	cargo clean

run:
	cargo run --release

bench:
	cargo bench
