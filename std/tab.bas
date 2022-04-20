000 rem tab.bas standard library function
025 function x
050 let output = ""
055 if x < 1 then 400
075 if x < pr_loc then 400
080 let last = x - pr_loc
100 for i = 1 to last
200 let output = output + " "
300 next i
400 function return output
