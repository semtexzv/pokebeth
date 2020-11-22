#!/usr/bin/env bash

NAME=${1:-charizard}

curl -v localhost:5000/pokemon/${NAME}