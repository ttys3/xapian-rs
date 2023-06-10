# create doc branch: git checkout --orphan doc-1.5
doc:
	cargo doc --no-deps --document-private-items -F vendored-xapian
	echo "see generated documentation under: $(shell pwd)target/doc/xapian/ffi/"

test: export LD_LIBRARY_PATH=xapian/xapian-core/.libs
test:
	cargo test --package xapian -F vendored-xapian --lib test::test_xapian -- --exact

test_wrapper: export LD_LIBRARY_PATH=xapian/xapian-core/.libs
test_wrapper:
	cargo test --package xapian -F vendored-xapian --lib test::test_xapian_wrapper -- --exact