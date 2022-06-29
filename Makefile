VERSION = $(shell awk -F ' = ' '$$1 ~ /^version/ { gsub(/[\\"]/, "", $$2); printf("%s",$$2) }' Cargo.toml)

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

release: build
	docker tag standup:$(VERSION) registry.heroku.com/standup-nw/web
	docker push registry.heroku.com/standup-nw/web
	heroku container:release web -a standup-nw

release_ta: build
	docker tag standup:$(VERSION) registry.heroku.com/standup-ta/web
	docker push registry.heroku.com/standup-ta/web
	heroku container:release web -a standup-ta
