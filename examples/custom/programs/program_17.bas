000 let a = 0
010 let b = 10
020 let c = -10
030 let d = 15
050 let e = 35
100 if a < b then 130
110 print "FAIL <"
120 STOP
130 print "PASS <"
200 if e > b then 230
210 print "FAIL >"
220 STOP
230 print "PASS >"
300 if 10 = (10+10-10) then 330
310 print "FAIL ="
320 STOP
330 print "PASS ="
400 if 10 <> 3.14159 then 430
410 print "FAIL <>"
420 STOP
430 print "PASS <>"
500 if -5 <= -5 then 508
502 print "FAIL <=, = PART"
505 STOP
508 if c <= 3.14159 then 530
510 print "FAIL <=, < PART"
520 STOP
530 print "PASS <=, BOTH PARTS"
600 if 3.14159 >= 3.14159 then 608
602 print "FAIL >=, = PART"
605 STOP
608 if 54 >= 3.14159 then 630
610 print "FAIL >=, > PART"
620 STOP
630 print "PASS >=, BOTH PARTS"
