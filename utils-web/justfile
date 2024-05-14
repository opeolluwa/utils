@_list:
	just --list --unsorted


alias r := run

bt := '0'

log := "warn"

export JUST_LOG := log

watch:
	cargo watch -c -- just verify
	
run:
   trunk serve

test:
    cargo test --target wasm32-unknown-unknown

# Perform all verifications (compile, test, lint etc.)
verify: test lint

# Run the static code analysis
lint:
	cargo fmt --check

clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules


fmt:
  cargo fmt


install-tailwindcss:
	curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
	chmod +x tailwindcss-linux-x64
	mv tailwindcss-linux-x64 /usr/bin/tailwindcss