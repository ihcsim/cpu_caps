check:
	cargo fmt -- --check
	cargo check

deps:
	cargo machete --fix || true

run:
	cargo run

test:
	cargo test -- --nocapture

release:
	cargo build --release

xml_to_rs:
	for mod in virsh_domcapabilities capabilities supported_features; do \
		rm -rf ./src/de/types/$$mod.rs; \
		xml_schema_generator -d "Serialize, Deserialize, Debug, PartialEq, Clone, Default" ./testdata/$$mod.xml ./src/de/types/$$mod.rs; \
	done

tools:
	cargo install -f xml_schema_generator --features="env_logger"

kind:
	kind create cluster --name dev --config=./kind.yaml

purge:
	kind delete cluster --name dev

kubevirt:
	VERSION=$$(curl -s https://storage.googleapis.com/kubevirt-prow/release/kubevirt/kubevirt/stable.txt) ;\
	kubectl create -f "https://github.com/kubevirt/kubevirt/releases/download/$${VERSION}/kubevirt-operator.yaml" ;\
	kubectl create -f "https://github.com/kubevirt/kubevirt/releases/download/$${VERSION}/kubevirt-cr.yaml"
