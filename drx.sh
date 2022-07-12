#!/bin/bash/
#: Description  : recipie to install other packages
# Author : <Adeoye Adefemi adefemiadeoye@yahoo.com>
#: Version : 1.0
#: Date : 12/07/2022
#: Usage : utils install <util name>
#: Options : none
#-----------------------------------------------------------
 
 #split the file from the line containing the package name and the file extension
 # eg  git-upadte.sh becomes git-update
 IFS=. read -r var1 var2 <<< $2

 #install the package
 touch $var1 
 cat $2 > $var1

sudo mv $var1 /usr/local/bin/
 echo "$var1 installed"