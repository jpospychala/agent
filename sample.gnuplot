set term svg size 1024,1536 background rgb 'white'
set datafile missing "-"
set style data linespoints
set xlabel "Time (ms)"

set multiplot layout 2,1

set ylabel "RSS (MB)"
plot for [col=2:*:2] 'sample.dat' using 0:col with lines title columnheader

set ylabel "CPU (%)"
plot for [col=3:*:2] 'sample.dat' using 0:col with lines title columnheader

# set ylabel "I/O"
# plot for [col=2:*:2] 'sample.dat' using 0:col with lines title columnheader

# set ylabel "Disk use"
# plot for [col=2:*:1] 'sample.dat' using 0:col with lines title columnheader

unset multiplot
