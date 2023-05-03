CHATGPT=./target/release/rust-chatgpt-cli
TEMPLATE=scripts/rust/code_review/review.template

GIT_DIFF_BUFFER=/tmp/gitdiff_buffer
PROMPT_BUFFER=/tmp/codereview_prompt

# get the current cached diff
# echo "get git diff"
git diff --cached > $GIT_DIFF_BUFFER

# echo "copy template to prompt"
cat $TEMPLATE > $PROMPT_BUFFER
# echo "copy git diff to prompt"
cat $GIT_DIFF_BUFFER >> $PROMPT_BUFFER

cat $PROMPT_BUFFER | $CHATGPT
