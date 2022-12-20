ip-setup:
	cd .data && 7z x ips.7z
ip-run:
	cargo build --release
	time ./target/release/ipxmas ip
ip-web:
	cargo build --release
	./target/release/ipxmas ip-web
ip-mem:
	deno run -A test/ip.ts --mem
ip-benchmark:
	deno run -A test/ip.ts --refresh
	deno run -A test/ip.ts --ips10
ip-curl:
	/usr/bin/time curl localhost:8668/refresh
	/usr/bin/time curl localhost:8668/ips?value=123.123.132.123,123.123.132.123





