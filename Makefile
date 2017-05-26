default: build
hard: test

CRATE = bf
REPO  = bf-rs

doc:
	rustup run nightly cargo doc --no-deps -p $(CRATE) --features=jit
	echo "<meta http-equiv='refresh' content='0;url=$(CRATE)/'>" > target/doc/index.html

upload-doc:
	make doc
	ghp-import -n target/doc
	git push -f https://github.com/tov/$(REPO).git gh-pages

release:
	scripts/prepare_release.sh $(VERSION)
	make upload-doc
	cargo publish

clean:
	cargo clean
	$(RM) src/raw.rs
