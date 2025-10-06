default: dev

dev:
	cargo leptos watch

install:
	pnpm install

jspackages-install:
	pnpm install --prefix ./jspackages

jspackages-build:
	pnpm run --prefix ./jspackages build
