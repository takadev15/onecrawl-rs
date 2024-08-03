#!/bin/bash

# Set env for parser
export PARSER_MAX_THREADS="20"
export PARSER_DURATION_SECONDS="100"
export MEMORY_PROFILER_LOG=warn

# Run parser first
LD_PRELOAD=/usr/lib/libbytehound.so ./target/release/onecrawl-parser >> logs/parser.log &

# Wait for crawler to run
sleep 10

# Set env for scouter
export CRAWLER_START_URLS="https://detik.com https://www.kompas.com https://www.bola.net"
export CRAWLER_MAX_THREADS="20"
export CRAWLER_DURATION_SECONDS="100"
export CRAWLER_KEYWORD="" 
export MEMORY_PROFILER_LOG=warn

LD_PRELOAD=/usr/lib/libbytehound.so ./target/release/onecrawl-scouter >> logs/scouter.log
