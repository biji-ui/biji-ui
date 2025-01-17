default: dev

dev:
	cargo leptos watch

tailwind:
	npx tailwindcss -i ./main.scss -o ./style/main.css

tailwind-watch:
	npx tailwindcss -i ./main.scss -o ./style/main.css --watch
