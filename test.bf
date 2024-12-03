this is brainfuck b!
increase the value at pointer 0 by 5
+++++
start a loop
[
    go one step foward
    >
    increase the cell by 10
    ++++++++++
    go back one step
    <
    decrease our counter by 1
    -
]
at the end of the loop; once the counter is set to 0
move the pointer one bit
>
increase the value at the current pointer by 2
++
print the output
.
decrease the value by 2
--
print the ascii output
.