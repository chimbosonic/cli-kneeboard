verbosity=

test:
	cargo run -- ${verbosity} -c ./__test__/test_checklists/test1.md
test_load_save:
	cargo run -- ${verbosity} -s -c ./__test__/test_checklists/test1.md

install_hooks:
	cp ./hooks/pre-commit .git/hooks/
	chmod +x .git/hooks/pre-commit