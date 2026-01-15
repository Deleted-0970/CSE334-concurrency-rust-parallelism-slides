#!/bin/bash

echo "counter" > results.csv

for i in {0..3000} ; do
    ./a.out | cut -c 13- >> results.csv
done
