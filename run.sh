#!/bin/sh

./rs-cpu-usage |
	cut -d. -f1 |
	sed 's/$/%/'
