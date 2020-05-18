set term svg size 1024,1536 background rgb 'white'
set datafile missing "-"
set style data linespoints
set xlabel "Time (ms)"

set multiplot layout 4,1

set ylabel "RSS (MB)"
plot for [col=2:*:4] 'sample.dat' using 0:col with lines title columnheader

set ylabel "CPU (%)"
plot for [col=3:*:4] 'sample.dat' using 0:col with lines title columnheader

set ylabel "I/O Read"
plot for [col=4:*:4] 'sample.dat' using 0:col with lines title columnheader

set ylabel "I/O Write"
plot for [col=5:*:4] 'sample.dat' using 0:col with lines title columnheader

# set ylabel "Disk use"
# plot for [col=2:*:1] 'sample.dat' using 0:col with lines title columnheader

unset multiplot
