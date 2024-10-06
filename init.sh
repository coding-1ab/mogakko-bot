#!/bin/sh
if [ ! -f "/app/mogakko.db" ]; then
	cp "/ro/mogakko.db" "/app/mogakko.db"
fi
