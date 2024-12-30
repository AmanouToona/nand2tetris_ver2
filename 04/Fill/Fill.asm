// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed,
// the screen should be cleared.

(LOOP)
// set color to draw
// white
@color
M=0
@KBD
D=M
@INITIALIZE
D;JEQ

// black
@color
M=-1

// draw screen
(INITIALIZE)
@SCREEN
D=A
@8192  // 8K
D=D+A
@pos
M=D

(DRAW)
@color
D=M
@pos
A=M
M=D
@pos
M=M-1
D=M
@SCREEN
D=D-A

// unfinish draw screen
@DRAW
D;JGE

@LOOP
0;JEQ
