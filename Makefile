check:
	cargo fmt -- --check
	cargo check

deps:
	cargo machete --fix || true

run:
	cargo run

test:
	cargo test -- --nocapture

xml_to_rs:
	for mod in virsh_domcapabilities capabilities supported_features; do \
		rm -rf ./src/de/types/$$mod.rs; \
		xml_schema_generator -d "Serialize, Deserialize, Debug, PartialEq" ./testdata/$$mod.xml ./src/de/types/$$mod.rs; \
	done

tools:
	cargo install -f xml_schema_generator --features="env_logger"
