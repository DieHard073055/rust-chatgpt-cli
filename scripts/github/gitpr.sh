CHATGPT=./target/release/rust-chatgpt-cli
TEMPLATE=scripts/github/templates/gitpr.template

GIT_DIFF_BUFFER=/tmp/gitdiff_buffer
PROMPT_BUFFER=/tmp/gitpr_prompt

cat $TEMPLATE > $PROMPT_BUFFER
cat $GIT_DIFF_BUFFER >> $PROMPT_BUFFER

cat $PROMPT_BUFFER | $CHATGPT
