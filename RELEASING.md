# Releasing

Every package carries the same version, so a version names one set of prefix rules. Release from an up-to-date `main`.

## 1. Check

```sh
./vendor/bin/pest
(cd js && bun install && bun test)
(cd python && uv run pytest)
(cd rust && cargo test)
(cd go && go test ./...)
(cd dart && dart test)
git archive HEAD | tar -t   # PHP download: only LICENSE, composer.json, readme.md, src/
```

## 2. Set the version

Set it in `js/package.json`, `python/pyproject.toml`, `rust/Cargo.toml` and `dart/pubspec.yaml`, then commit. PHP and Go take theirs from the tag.

## 3. Publish

| Package | Command | Login (once) |
|---|---|---|
| Packagist `malico/mobile-cm-php` | `git tag 1.4.1 && git push origin 1.4.1` | none; Packagist picks up the tag |
| Go `github.com/yondifon/mobile-cm/go` | `git tag go/v1.4.0 && git push origin go/v1.4.0` | none |
| npm `mobile-cm` | `cd js && npm publish` | `npm login` |
| PyPI `mobile-cm` | `cd python && rm -rf dist && uv build && uv publish` | a pypi.org API token, given to `uv publish --token` or `UV_PUBLISH_TOKEN` |
| crates.io `mobile-cm` | `cd rust && cargo publish` | `cargo login` with a crates.io token |
| pub.dev `mobile_cm` | `cd dart && dart pub publish` | `dart pub login` |

PHP tags have no `v` prefix (`1.4.1`). Go tags need the folder prefix (`go/v1.4.0`), and Packagist ignores them.

A published version can't be replaced on any registry. To fix a release, publish the next patch version.
