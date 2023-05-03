CHATGPT=./target/release/rust-chatgpt-cli
TEMPLATE=scripts/github/templates/gitcommit.template

GIT_DIFF_BUFFER=/tmp/gitdiff_buffer
PROMPT_BUFFER=/tmp/gitcommit_prompt

# get the current cached diff
# echo "get git diff"
git diff --cached > $GIT_DIFF_BUFFER

# echo "copy template to prompt"
cat $TEMPLATE > $PROMPT_BUFFER
# echo "copy git diff to prompt"
cat $GIT_DIFF_BUFFER >> $PROMPT_BUFFER

cat $PROMPT_BUFFER | $CHATGPT
