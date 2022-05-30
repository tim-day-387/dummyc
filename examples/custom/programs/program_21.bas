001 A = 5 : B = 4 : print A + B
002 for i = 0 to 5
003 print "foo"
004 next i
005 for i = 0 to 5 : print "bar" : next i
006 j = 0
007 print "foobar" : if j = 5 then 008 : j = j + 1 : goto 007
008 print "done"
009 end