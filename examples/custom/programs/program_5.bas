1000 REM This is a test of the REM and STOP functions
1500 PRINT "Hello!"
1750 REM Set test 1
2000 LET Test1="Sample A"
2500 REM Set test 2
3000 LET Test2="Sample B"
3500 REM Print both variables
4000 PRINT Test1
5000 PRINT Test2
6000 GOSUB 8000
7000 GOSUB 9000
7500 STOP
8000 REM Beginning of FIRST sub
8100 IF Test1="Sample A" THEN 8400
8200 PRINT "Path A!"
8300 RETURN
8400 PRINT "Path B!"
8500 RETURN
9000 REM Beginning of SECOND sub
9100 IF Test2="Sample B" THEN 9200
9200 PRINT "Path A!"
9300 RETURN
9400 PRINT "Path B!"
9500 RETURN
10000 END