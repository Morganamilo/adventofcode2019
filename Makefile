SRC = $(wildcard src/*.rs)
BIN = $(SRC:.rs=)
OPTLEVEL=0

build: $(BIN)


check: $(BIN:src/%=check-%)

fmt: $(BIN:src/%=fmt-%)

%: %.rs
	rustc $< -g -C opt-level=$(OPTLEVEL) -o $@

check-%: src/%.rs
	clippy-driver src/$(@:check-%=%).rs -o $(@:check-%=src/%)

fmt-%: src/%.rs
	rustfmt src/$(@:fmt-%=%).rs

run-%-1: src/%-1
	./$< < in/$(@:run-%-1=%)

run-%-2: src/%-2
	./$< < in/$(@:run-%-2=%)

run-stdin-%: src/%
	./$<

clean:
	find src/ -type f  ! -name "*.*"  -delete
