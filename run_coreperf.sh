#!/bin/bash

# Set env for parser
export PARSER_MAX_THREADS="20"
export PARSER_DURATION_SECONDS="100"

# Run parser first
flamegraph -o benchgraph/parsergraph.svg -- ./target/release/onecrawl-parser >> logs/parser.log&

# Wait for crawler to run
sleep 10

# Set env for parser
export CRAWLER_START_URLS="https://detik.com"
export CRAWLER_MAX_THREADS="20"
export CRAWLER_DURATION_SECONDS="100"
export CRAWLER_KEYWORD="" 

flamegraph -o benchgraph/parsergraph.svg -- ./target/release/onecrawl-scouter >> logs/scouter.log
