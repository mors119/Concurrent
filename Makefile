run-java:
	make -C java run ARGS="$(ARGS)"

run-c:
	make -C c run ARGS="$(ARGS)"

run-rust:
	cd rust && cargo run $(ARGS)