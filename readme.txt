perf stat -ad -r 100 target/debug/advent_of_code
perf record --call-graph dwarf target/debug/advent_of_code
hotspot perf.data