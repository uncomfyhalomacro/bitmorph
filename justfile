#!/usr/bin/just

get-dependencies:
	sudo zypper -vvv cc
	sudo zypper -vvv --non-interactive dup
	sudo zypper -vvv in gdk-pixbuf-devel

prebuild:
	dfx generate

build: clean frontend

local-deploy: build
	dfx deploy

ic-deploy:
	dfx deploy --network=ic

generate:
	dfx generate bitmorph_backend

frontend:
	#!/bin/bash
	cd src/bitmorph_frontend/
	dx build --release

clean:
	rm -rfv target/
	rm -rfv .dfx
