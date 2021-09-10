BACKEND_VERSION=$(shell sh -c "cd ../standup; cargo get version --full")
TARGET="standup"

build: clean
            docker build --no-cache -t $(TARGET):$(BACKEND_VERSION) .

clean:
        -docker rmi $(TARGET):$(BACKEND_VERSION)

upload: build
        #docker push registry.digitalocean.com/registry-worldmill-online/worldmill/$(TARGET):$(BACKEND_VERSION)