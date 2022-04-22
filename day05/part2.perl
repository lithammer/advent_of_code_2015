#!/usr/bin/perl -wnl

next unless /(..).*\1/;
next unless /(.).\1/;

$count++;

END{print $count}
