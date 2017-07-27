CARGO = cargo
RM = rm -rf

build:
	$(CARGO) build

test:
	$(CARGO) test

doc:
	$(CARGO) doc

update-gh-pages:
	git pull origin master
	$(RM) target
	mkdir -p target/doc
	git worktree prune
	git worktree add target/doc gh-pages
	cd target/doc && git reset --hard HEAD~1
	cargo doc
	cd target/doc && git add .
	cd target/doc && git commit -m "updates gh-pages"
	git push -f origin gh-pages

