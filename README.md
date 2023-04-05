cargo build && ./maelstrom test -w echo --bin ../echo.sh --node-count 1 --time-limit 10 --log-stderr


cargo build && ./maelstrom test -w unique-ids --bin ../echo.sh --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition --log-stderr
