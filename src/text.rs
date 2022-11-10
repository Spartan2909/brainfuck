pub static HELP_GENERAL: &str = "Brainfuck uses the following characters: '>', '<', '+', '-', '.', ',', '[', and ']'.
Any other characters are ignored. 

At the start of the program, an array of 65536 bytes is created, along with a pointer for that array. 
The previously mentioned characters are used to manipulate that array and pointer. 

Definitions: 
> : Increment the data pointer.
< : Decrement the data pointer.
+ : Increment the value at the data pointer.
- : Decrement the value at the data pointer.
. : Output the value at the data pointer, encoded as ASCII.
, : Accept one byte of input, storing its ASCII code point at the data pointer. 
[ : If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
] : If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

More information: https://wikipedia.org/wiki/Brainfuck

Error types: overflow, syntax, file handling, parsing";

pub static HELP_OVERFLOW: &str = "An Overflow Error occurs when a value is too high or too low. If the error occurred at the data pointer, it means that the pointer was moved too far to the left (underflow) or too far to the right (overflow). If it occurred at an array index, it means that the value was set to below zero (underflow) or above 255 (overflow).";

pub static HELP_SYNTAX: &str = "A Syntax Error means that there was a problem with the supplied program that made it invalid. If it occurred from a mismatched bracket, you should check that the program contains the same number of opening and closing brackets.";

pub static HELP_FILE: &str = "A File Handling Error means that there was a problem reading the data from the supplied file. Ensure that the file exists and you have permission to access it.";

pub static HELP_PARSING: &str = "A Parsing Error means that there was a problem interpreting a character from the supplied file. Ensure that all characters are valid Unicode characters.";

pub static HELP_INTERNAL: &str = "An Internal Error means that there was a problem with the application. Please submit a bug report at https://github.com/Spartan2909/brainfuck/issues/new?labels=bug&template=bug_report.md";

pub static HELP_ITER: &str = "An iteration error occurs when a loop is executed too many times. Check your code for possible infinite loops.";

pub static INFO: &str = "Brainfuck is an esoteric programming language originally developed by Urban Müller in 1993.
This is my implementation of it in Rust.";
