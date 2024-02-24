#!/bin/bash
git fetch --all
for BRANCH in $(git branch -a | grep origin/dependabot | cut -d '/' -f 3-)
do
    git switch $BRANCH
    git rebase main
    if [ $? -ne 0 ]; then
        echo "Rebase failed for $BRANCH. resolve conflicts and run 'git rebase --continue'"
        read -p "Press enter when done"
    fi
    git push --force
    git switch main
    git merge $BRANCH
    git push -d origin $BRANCH
    git branch -d $BRANCH
done
git rebase -i origin/main