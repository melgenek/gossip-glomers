build:
	cargo build

.PHONY: maelstrom_echo
maelstrom_echo: build
	(cd ./maelstrom && ./maelstrom test -w echo --bin ../target/debug/echo --node-count 1 --time-limit 10 --log-stderr)

.PHONY: maelstrom_unique_id
maelstrom_unique_id: build
	(cd ./maelstrom && ./maelstrom test -w unique-ids --bin  ../target/debug/unique_id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition --log-stderr)

.PHONY: maelstrom_broadcast_nemesis
maelstrom_broadcast_nemesis: build
	(cd ./maelstrom && ./maelstrom test -w broadcast --bin  ../target/debug/broadcast --node-count 25 --time-limit 20 --rate 100 --latency 100 --nemesis partition --log-stderr)

.PHONY: maelstrom_broadcast_efficient
maelstrom_broadcast_efficient: build
	(cd ./maelstrom && ./maelstrom test -w broadcast --bin  ../target/debug/broadcast --node-count 25 --time-limit 20 --rate 100 --latency 100 --log-stderr)

.PHONY: maelstrom_broadcast_simple
maelstrom_broadcast_simple: build
	(cd ./maelstrom && ./maelstrom test -w broadcast --bin  ../target/debug/broadcast --node-count 25 --time-limit 20 --rate 100 --log-stderr)

.PHONY: maelstrom_serve
maelstrom_serve:
	(cd ./maelstrom && ./maelstrom serve)
