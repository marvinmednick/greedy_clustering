#!/bin/bash

for f in $*
do
	outputFile=${f/input/output}
	result=`cluster $f 3 --hamming`
	answer=`cat $outputFile`
	if [[  "$result" == "$answer" ]]
	then 
		echo "$f PASSED $result $answer"
	else 
		echo "$f FAILED $result $answer"
	fi
done
