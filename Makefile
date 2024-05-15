test:
	echo "Remember why you started!"

# the appliaction depnds on rust, seaorm cli and deno
install-toolchain:

run-server:
	 cargo run --manifest-path utils-server/Cargo.toml 

build-server:

watch-server:
	 @cd utils-server && cargo shuttle run --port 5000

watch-cli:
	cargo watch -qcx run '--manifest-path utils-cli/Cargo.toml'

build-cli:

deploy-web:
	cd utils-web
	rm -rf dist\
	trunk build --release
	surge