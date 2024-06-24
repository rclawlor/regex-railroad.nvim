#!/bin/bash

$URL = https://github.com/rclawlor/regex-railroad.nvim/releases/download/$2/regex-railroad

output=$(wget -q -O $1/regex-railroad $URL && chmod a+x $1/regex-railroad)
exit $?

