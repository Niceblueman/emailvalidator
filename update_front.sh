#!/bin/bash
if [ -d './template' ]
then
    rm -rf ./template
fi
wget https://github.com/Niceblueman/emailvalidator/releases/download/latest/front.tar.gz
tar -xf front.tar.gz
rm front.tar.gz