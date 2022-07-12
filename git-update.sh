#!/bin/bash/
#: Description  : recipei to update git repositories wiht latest commit 
# Author : <Adeoye Adefemi adefemiadeoye@yahoo.com>
#: Version : 1.0
#: Date : 12/07/2022
#: Usage : git-update <message content>
#: Options : none
#--------------------------------------------------------

git add .
git commit -m "UPDATE -> $1"
git push
echo "Successfully updated"

