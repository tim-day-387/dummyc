010 for i = 0 to 4
020 for j = 0 to 3
030 if i = 0 then 080
040 if i = 2 then 080
050 if i = 4 then 080
060 print "Indices: i = ", i, ", j = ", j, ", i is odd"     
070 next j
075 goto 100
080 print "Indices: i = ", i, ", j = ", j, ", i is even"    
090 next j
100 next i
