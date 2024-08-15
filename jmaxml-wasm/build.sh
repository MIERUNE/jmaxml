#!/bin/bash

wasm-pack build -t web
python patch_esm.py
