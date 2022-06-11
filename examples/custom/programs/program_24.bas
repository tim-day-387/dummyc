100 option base 0
200 dim a(10)
300 for i = 0 to 9
400 print "Index: a:" ; i : a(i) = 2.5 : print "=> value:" ; a(i)
450 next i
500 option base 1
550 dim b(10)
600 for i = 1 to 10
700 print "Index: b:" ; i : b(i) = 5 : print "=> value:" ; b(i)
750 next i
1000 option base 1+1*3
1550 dim c(10)
1600 for i = (1+1*3) to (10+(1+1*3))
1700 print "Index: c:" ; i : c(i) = 7.5 : print "=> value:" ; c(i)
1750 next i
