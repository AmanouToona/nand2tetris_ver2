// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// The algorithm is based on repetitive addition.

@ans
M=0

// cnt
@R1
D=M
@cnt
M=D

(LOOP)
//if (cnt == 0) goto END
@cnt
D=M
D;JEQ

// ans += R0
@R0
D=M
@ans
M=D+M

// cnt -= 1
@cnt
M=M-1

// goto LOOP
@LOOP
0;JMP

(END)
// save answer to R2
@ans
D=M
@R2
M=D

@END
0;JMP