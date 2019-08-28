RUSTFLAGS="-Clink-arg=-fuse-ld=gold" cargo afl build --release
AFL_SKIP_CPUFREQ=1 cargo afl fuzz -i in -o out target/release/fuzz

# Replaced /proc/sys/kernel/core_pattern
# from:	|/usr/share/apport/apport %p %s %c %d %P
# to:  	core
