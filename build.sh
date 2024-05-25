#!/bin/bash

output=$(wget -q -O $1/regex-railroad https://github.com/rclawlor/regex-railroad.nvim/releases/download/$2/regex-railroad && chmod a+x $1/regex-railroad)
exit $?

