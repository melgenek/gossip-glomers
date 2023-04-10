build:
	cargo build

.PHONY: maelstrom_echo
maelstrom_echo: build
	(cd ./maelstrom && ./maelstrom test -w echo --bin ../target/debug/echo --node-count 1 --time-limit 10 --log-stderr)

.PHONY: maelstrom_unique_id
maelstrom_unique_id: build
	(cd ./maelstrom && ./maelstrom test -w unique-ids --bin  ../target/debug/unique_id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition --log-stderr)

.PHONY: maelstrom_broadcast
maelstrom_broadcast: build
	(cd ./maelstrom && ./maelstrom test -w broadcast --bin  ../target/debug/broadcast --node-count 5 --time-limit 20 --rate 10 --nemesis partition --log-stderr)

.PHONY: maelstrom_serve
maelstrom_serve:
	(cd ./maelstrom && ./maelstrom serve)
