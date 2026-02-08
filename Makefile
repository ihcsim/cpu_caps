check:
	cargo fmt -- --check
	cargo check

crates:
	cargo add quick-xml --features serialize,async-tokio,encoding

run:
	cargo run

test:
	cargo test -- --nocapture

rsstruct:
	xml_schema_generator ./testdata/virsh_domcapabilities.xml src/schema/mod.rs

tools:
	cargo install -f xml_schema_generator --features="env_logger"
