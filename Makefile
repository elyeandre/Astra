pack_lua:
	@cd lua && luajit pack.lua astra.lua

# use this syntax: just release_tag="0.0.0" range="start..end" update-changelog
update-changelog:
    @git cliff --unreleased --tag="$(TAG)" --prepend CHANGELOG.md