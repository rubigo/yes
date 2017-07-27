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
	cd gh-pages && $(RM) * .*
	[ "`git branch --list gh-pages`" ] && git branch -D gh-pages
	cd gh-pages && git checkout --orphan gh-pages
	cp -r target/doc/* gh-pages
	cd gh-pages && git add . && git commit -m "updates gh-pages"
	$(RM) gh-pages
	git push -f origin gh-pages

