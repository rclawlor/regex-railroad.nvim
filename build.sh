#!/bin/bash

output=$(wget -O regex-railroad https://github.com/rclawlor/regex-railroad.nvim/releases/download/$1/regex-railroad)
exit $?

