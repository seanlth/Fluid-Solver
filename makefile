
build:
	clang -c src/linear_solver.c
	ar rvs libsolver.a linear_solver.o
	rm linear_solver.o
	mv libsolver.a target/debug/deps/
	cargo build


clean:
	cargo clean

run:
	cargo run
