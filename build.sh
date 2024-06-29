#!/bin/bash

URL=https://github.com/rclawlor/regex-railroad.nvim/releases/download/$2/regex-railroad-$3

output=$(wget -q -O $1/regex-railroad-$3 $URL && chmod a+x $1/regex-railroad-$3)
exit $?

