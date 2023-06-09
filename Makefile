example/hello: export XAPIAN_DEBUG_LOG=-
example/hello: export LD_LIBRARY_PATH=xapian/xapian-core/.libs
example/hello:
	cargo run --example hello -F vendored-xapian

example/index: export LD_LIBRARY_PATH=xapian/xapian-core/.libs
example/index:
	cargo run --example index -F vendored-xapian

example/search: export XAPIAN_DEBUG_LOG=-
example/search: export LD_LIBRARY_PATH=xapian/xapian-core/.libs
example/search:
	cargo run --example search -F vendored-xapian