VERSION = $(shell awk -F ' = ' '$$1 ~ /version/ { gsub(/[\"]/, "", $$2); printf("%s",$$2) }' Cargo.toml)

build:
	docker build -t standup:$(VERSION) .

clean:
	docker rmi standup:$(VERSION)

rebuild: clean
	docker build --no-cache -t standup:$(VERSION) .

run:
	docker container run -p 4200:4200 standup:$(VERSION)

start: build run

restart: rebuild run

prune:
	docker container prune --filter "label=name=standup"
