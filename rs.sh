#!/bin/sh

razplus-search --name "$1" | rg -i "$1"
