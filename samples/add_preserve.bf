+++               Set Cell #0 to 3
>++               Set Cell #1 to 2
<                 Move to Cell #0
[                 Copy the value in Cell #0 to Cells #2 and #3
  >>+             Add 1 to Cell #2
  >+              Add 1 to Cell #3
  <<<-            Subtract 1 from Cell #0
]
>                 Move to Cell #1
[                 Copy the value from Cell #1 to Cells #2 and #4
  >+              Add 1 to Cell #2
  >>+             Add 1 to Cell #4
  <<<-            Subtract 1 from Cell #1
]
>>                Move to Cell #3
[                 Move the temporary value from Cell #3 to Cell #0
  <<<+            Add 1 to Cell #0
  >>>-            Subtract 1 from Cell #3
]
>                 Move to Cell #4
[                 Move the temporary value from cell #4 to Cell #1
  <<<+            Add 1 to Cell #1
  >>>-            Subtract 1 from Cell #4
]
<++++++           Set Cell #3 to 6; it was set to 0 in a previous loop
[                 Add 48 to Cell #2 to get the ASCII character for the number
  >++++++++       Set Cell #4 to 8; it was set to 0 in a previous loop
  [
    <<+           Add 1 to Cell #2
    >>-           Subtract 1 from Cell #4
  ]
  <-              Subtract 1 from Cell #3
]
<.                Output the value at cell #2
