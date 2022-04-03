010 FOR I=0 TO 4
020 FOR J=0 TO 3
030 IF I=0 THEN 080
040 IF I=2 THEN 080
050 IF I=4 THEN 080
060 PRINT "Indices: I=", I, ", J=", J, ", I is odd"     
070 NEXT J
075 GOTO 100
080 PRINT "Indices: I=", I, ", J=", J, ", I is even"    
090 NEXT J
100 NEXT I
