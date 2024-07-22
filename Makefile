dev:
	docker-compose -f docker-compose.yml up -d

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

new-migrate:
	sqlx migrate add -r $(name)

ngrok:
	ngrok http --domain=quietly-credible-cattle.ngrok-free.app 8093

prod:
	docker compose down
	git pull
	docker compose up -d

server:
	cargo watch -q -c -w src/ -x run

server-up:
	ngrok http 8090
