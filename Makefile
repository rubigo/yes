CARGO = cargo
RM = rm -rf

build:
	$(CARGO) build

test:
	$(CARGO) test

doc:
	$(CARGO) doc

update-gh-pages: doc
	$(RM) gh-pages
	git clone . gh-pages
	cd gh-pages && $(RM) * && git checkout --orphan gh-pages
	cp -r target/doc/* gh-pages
	cd gh-pages && git commit -am "updates gh-pages"
	$(RM) gh-pages
	git push -f origin gh-pages

