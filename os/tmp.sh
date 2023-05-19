#!/bin/bash

for i in {1..30}
do
    make run-fat32 > tmp/tmp$i.log
done