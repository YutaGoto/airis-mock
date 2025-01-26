SHELL := /bin/bash
MAKEFLAGS = $(shell [ "$(debug)" == "true" ] || echo "s" )

.PHONY: run
run:
	docker compose up

.PHONY: stop
stop:
	docker compose down

.PHONY: clean
clean:
	docker compose down --volumes --remove-orphans
