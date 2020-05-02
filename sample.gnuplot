set datafile missing "-"
set term png size 1024,768
set style data linespoints

set key outside
plot for [col=2:*] 'sample.dat' using 0:col with lines title columnheader