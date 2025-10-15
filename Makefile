default: dev

dev: jspackages-build
	trunk serve

install:
	pnpm install

dev-install:
	cargo install rustywind

jspackages-install:
	pnpm install --prefix ./jspackages

jspackages-build:
	pnpm run --prefix ./jspackages build
