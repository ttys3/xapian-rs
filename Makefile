doc:
	cargo doc --document-private-items -F vendored-xapian
	echo "see generated documentation under: $(shell pwd)target/doc/xapian/ffi/"

test:
	cargo test --package xapian -F vendored-xapian --lib test::test_xapian -- --exact

test_wrapper:
	cargo test --package xapian -F vendored-xapian --lib test::test_xapian_wrapper -- --exact