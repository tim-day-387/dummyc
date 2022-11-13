10 PRINT "PROGRAM FILE 194: ERROR - ASSIGNED QUOTED STRINGS "
15 PRINT "        CONTAINING SINGLE QUOTE."
20 PRINT "    ANSI STANDARD 3.2, 9.2"
30 PRINT
40 PRINT "SECTION 194.1: ERROR - ASSIGNED QUOTED STRINGS  "
45 PRINT "        CONTAINING SINGLE QUOTE."
50 PRINT
60 PRINT "THIS PROGRAM TESTS TO SEE IF THE PROCESSOR ACCEPTS PROGRAMS"
70 PRINT "CONTAINING A SINGLED OCCURRENCE OF THE QUOTE CHARACTER WITHIN"
80 PRINT "THE QUOTED STRING OF A LET-STATEMENT."
90 PRINT
100 PRINT "THIS IS A TEST FOR A NON-STANDARD FEATURE OF MINIMAL BASIC."
110 PRINT "TO PASS THIS TEST, THE PROCESSOR MUST EITHER:"
120 PRINT
130 PRINT "   1) ACCEPT THE PROGRAM AND BE ACCOMPANIED BY DOCUMENTATION"
140 PRINT "      ACCURATELY DESCRIBING THE FEATURE'S INTERPRETATION"
150 PRINT "      BY THE PROCESSOR, OR"
160 PRINT "   2) REJECT THE PROGRAM WITH AN APPROPRIATE ERROR MESSAGE"
180 PRINT
190 PRINT "SEE THE NBS MINIMAL BASIC TEST PROGRAMS USER'S MANUAL"
200 PRINT "FOR DETAILED CRITERIA."
210 PRINT
220 PRINT "                              BEGIN TEST."
230 PRINT
240 PRINT "ABOUT TO ASSIGN TO A$ A QUOTED-STRING CONTAINING:"
250 PRINT "ASTERISK, QUOTE, QUESTION-MARK."
260 LET A$="*"?"
270 PRINT "A$=";A$
310 PRINT
320 PRINT "                               END TEST."
330 PRINT
340 PRINT "END PROGRAM 194"
350 END