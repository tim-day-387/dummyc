10 PRINT "PROGRAM FILE 163: ERROR - REFERENCE TO AN UNDEFINED FUNCTION."
20 PRINT "    ANSI STANDARD 16.4"
30 PRINT
40 PRINT "SECTION 163.1: ERROR - REFERENCE TO AN UNDEFINED FUNCTION."
50 PRINT
60 PRINT "THIS IS A TEST FOR A NON-STANDARD FEATURE OF MINIMAL BASIC."
70 PRINT "TO PASS THIS TEST, THE PROCESSOR MUST EITHER:"
80 PRINT
90 PRINT "   1) ACCEPT THE PROGRAM AND BE ACCOMPANIED BY DOCUMENTATION"
100 PRINT "      ACCURATELY DESCRIBING THE FEATURE'S INTERPRETATION"
110 PRINT "      BY THE PROCESSOR, OR"
120 PRINT
130 PRINT "   2) REJECT THE PROGRAM WITH AN APPROPRIATE ERROR MESSAGE"
140 PRINT
150 PRINT "SEE THE NBS MINIMAL BASIC TEST PROGRAMS USER'S MANUAL"
160 PRINT "FOR DETAILED CRITERIA."
170 PRINT
180 PRINT "                            BEGIN TEST."
190 PRINT
200 PRINT "ABOUT TO ATTEMPT INVOCATION OF FNA, WHICH IS UNDEFINED."
210 LET A=FNA(1)
220 PRINT "PROCESSOR HAS EVALUATED FNA(1) = ";A
230 PRINT
240 PRINT "                             END TEST."
250 PRINT
260 PRINT "END PROGRAM 163"
270 END