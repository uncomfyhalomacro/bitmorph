#!/usr/bin/just

build: prebuild
	dx build

prebuild:
	dfx generate

local-deploy:
	dfx deploy --network=local

ic-deploy:
	dfx deploy --network=ic

generate:
	dfx generate bitmorph-backend

frontend:
	#!/bin/bash
	cd crates/bitmorph_frontend/
	dx build
