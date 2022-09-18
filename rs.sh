#!/bin/sh

razplus-search --name "$1" | rg "$1"
