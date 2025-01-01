pack_lua:
	@cd lua && luajit pack.lua astra.lua

update-changelog:
	@git cliff --unreleased --tag="$(TAG)" --prepend CHANGELOG.md