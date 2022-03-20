1000 REM This is a test of the REM and STOP functions
1500 PRINT "Hello!"
1750 REM Set test 1
2000 LET Test1="Sample A"
2500 REM Set test 2
3000 LET Test2="Sample B"
3500 REM Print both variables
4000 PRINT Test1
5000 PRINT Test2
6000 IF Test1="Sample A" THEN 7500
7000 STOP
7500 PRINT "Not Over A!"  
7750 LET Test3="Sample A"
8000 IF Test2=Test3 THEN 9500
9000 STOP
9500 PRINT "Not Over B!"
10000 END
