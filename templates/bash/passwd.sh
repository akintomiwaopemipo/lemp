#!/usr/bin/expect -f

set timeout -1

spawn passwd

expect "(current) UNIX password: "

send "hamdan \r"

expect eof