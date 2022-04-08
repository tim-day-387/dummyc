10 PRINT "PROGRAM FILE 7: EXCEPTION - STRING OVERFLOW USING"
15 PRINT "        THE LET-STATEMENT."
20 PRINT "    ANSI STANDARD 9.5, 12.4"
30 PRINT
40 PRINT "SECTION 7.1: EXCEPTION - STRING OVERFLOW USING"
50 PRINT "              THE LET-STATEMENT."
60 PRINT
70 PRINT "STRING OVERFLOW OCCURS WHEN THE VALUE ASSIGNED TO A STRING"
80 PRINT "VARIABLE CONTAINS MORE CHARACTERS THAN CAN BE RETAINED."
82 PRINT
85 PRINT "TO PASS THIS TEST:"
90 PRINT
95 PRINT "    1) A MESSAGE IDENTIFYING THE EXCEPTION MUST BE"
100 PRINT "       DISPLAYED AND EXECUTION MUST TERMINATE, OR"
105 PRINT
110 PRINT "    2) STRING OVERFLOW MUST NOT OCCUR"
120 PRINT
130 PRINT "                               BEGIN TEST."
140 PRINT
145 PRINT "ABOUT TO ASSIGN STRING OF 19 CHARACTERS - "
150 LET A$="?*******19********!"
155 PRINT "ABOUT TO ASSIGN STRING OF 20 CHARACTERS - "
160 LET B$="?********20********!"
165 PRINT "ABOUT TO ASSIGN STRING OF 30 CHARACTERS - "
170 LET C$="?*************30*************!"
175 PRINT "ABOUT TO ASSIGN STRING OF 40 CHARACTERS - "
180 LET D$="?******************40******************!"
185 PRINT "ABOUT TO ASSIGN STRING OF 50 CHARACTERS - "
190 LET E$="?***********************50***********************!"
195 PRINT "ABOUT TO ASSIGN STRING OF 58 CHARACTERS - "
200 LET F$="?***************************58***************************!"
202 PRINT "ALL ASSIGNMENTS COMPLETED."
205 PRINT
210 PRINT "?*******19********!"
220 PRINT A$
230 PRINT
240 PRINT "?********20********!"
250 PRINT B$
260 PRINT
270 PRINT "?*************30*************!"
280 PRINT C$
290 PRINT
300 PRINT "?******************40******************!"
310 PRINT D$
320 PRINT
330 PRINT "?***********************50***********************!"
340 PRINT E$
350 PRINT
360 PRINT "?***************************58***************************!"
370 PRINT F$
380 PRINT
390 PRINT "IF THE PAIRED LINES IN THE OUTPUT ARE IDENTICAL AND EACH"
400 PRINT "   LINE STARTS WITH A QUESTION MARK AND ENDS WITH AN"
410 PRINT "   EXCLAMATION POINT, THEN"
420 PRINT "*** TEST PASSED ***"
440 PRINT
450 PRINT "                             END TEST."
460 PRINT
470 PRINT "END PROGRAM 7"
480 END
