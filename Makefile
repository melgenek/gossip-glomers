build:
	cargo build

.PHONY: maelstrom_echo
maelstrom_echo: build
	./maelstrom/maelstrom test -w echo --bin ./target/debug/echo --node-count 1 --time-limit 10 --log-stderr

.PHONY: maelstrom_unique_id
maelstrom_unique_id: build
	./maelstrom/maelstrom test -w unique-ids --bin  ./target/debug/unique_id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition --log-stderr

.PHONY: maelstrom_serve
maelstrom_serve:
	./maelstrom/maelstrom serve
