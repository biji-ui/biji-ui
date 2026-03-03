default: dev

# SSR dev server (Axum + hydration)
dev: jspackages-build
	LEPTOS_TAILWIND_VERSION=v4.2.1 cargo leptos watch

# CSR dev server (Trunk, no server)
dev-csr: jspackages-build
	trunk serve

install:
	pnpm install

dev-install:
	cargo install rustywind

jspackages-install:
	pnpm install --prefix ./jspackages

jspackages-build:
	pnpm run --prefix ./jspackages build
