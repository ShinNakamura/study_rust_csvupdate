#! /bin/bash
unalias -a

test -d ./sample || mkdir ./sample
<./tests/members.csv cargo run -- "age,-" >./sample/expect-ageless.csv
diff ./tests/expect-ageless.csv ./sample/expect-ageless.csv
