all: build
	docker build -t phebe .

build:
	cd phebe && yarn build
