BACKEND_VERSION=$(shell sh -c "cd ../standup; cargo get version --full")

build:
	docker build --no-cache -t standup:$(BACKEND_VERSION) .

clean:
	docker rmi standup:$(BACKEND_VERSION)

rebuild: clean
	docker build --no-cache -t standup:$(BACKEND_VERSION) .

run:
	docker container run -p 4200:4200 standup:$(BACKEND_VERSION)

start: build run

restart: rebuild run

prune:
	docker container prune --filter "label=name=standup"
