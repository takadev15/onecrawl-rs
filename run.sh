#!/bin/bash

# Set env for parser
export PARSER_MAX_THREADS="20"
export PARSER_DURATION_SECONDS="10800"

# Run parser first
./target/release/onecrawl-parser >> logs/parser.log &

# Wait for crawler to run
sleep 10

# Set env for parser
export CRAWLER_START_URLS="https://detik.com https://www.kompas.com https://www.bola.net"
export CRAWLER_MAX_THREADS="20"
export CRAWLER_DURATION_SECONDS="10800"
export CRAWLER_KEYWORD="" 

./target/release/onecrawl-scouter >> logs/scouter.log
