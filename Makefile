default: doc

CRATE = bf
REPO  = bf-rs

doc:
	rustup run nightly cargo doc --no-deps -p $(CRATE) --features="jit llvm"
	echo "<meta http-equiv='refresh' content='0;url=$(CRATE)/'>" > target/doc/index.html

upload-doc:
	make doc
	ghp-import -n target/doc
	git push -f https://github.com/tov/$(REPO).git gh-pages
