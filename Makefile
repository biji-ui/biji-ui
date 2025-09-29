default: dev

dev:
	cargo leptos watch

install:
	pnpm install

jspackages-install:
	pnpm install --prefix ./jspackages

jspackages-build:
	pnpm run --prefix ./jspackages build

tailwind:
	npx tailwindcss -i ./main.scss -o ./style/main.css

tailwind-watch:
	npx tailwindcss -i ./main.scss -o ./style/main.css --watch
