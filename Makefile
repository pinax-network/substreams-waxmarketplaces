.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run: build
	substreams run -e wax.firehose.eosnation.io:9001 map_events -s 264901435 -t +1

.PHONY: gui
gui: build
	substreams gui -e wax.firehose.eosnation.io:9001 map_events -s 264901426 -t +10000