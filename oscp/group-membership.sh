#!/bin/bash
read user
read group

if [ -n "$(cut -d ":" -f 1 /etc/passwd | grep -x $user)" ]
then
user_found=true
fi

if [ -n "$(cut -d ":" -f 1 /etc/group | grep -x $group)" ]
then
group_found=true
fi

if [ -z "$user_found" -a -z "$group_found" ]
then
echo Both are not found - why are you even asking me this?
elif [ -z "$user_found" -o -z "$group_found" ]
then
echo One exists, one does not. You figure out which.
else

if [ -n "$(groups $user | cut -d ":" -f 2 | grep -w $group)" ]
then
echo Membership valid!
else
echo Membership invalid but available to join.
fi

fi