#!/bin/bash

echo Installing...

echo Making directory /var/rustgas 
sudo mkdir -pv /var/rustgas
sudo chmod 777 /var/rustgas
echo Ok!

echo AGA Tables Reference
echo Downloading...
git submodule update --remote
cp -Rv aga_tables /var/rustgas
echo Ok!

echo Finished
