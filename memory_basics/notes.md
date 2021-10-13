#Stack
    * region of the process memory soring variables for each function
    * for every function call a stack frame is allocated
    * the size of each variable has to be known at compile time 
    * when the function exits the stack frame is released
    * the memory occupied by the functions variables is automatically released
    * the size is limited
    * the variables are in the scope of the function

#Heap
    * region of process memory that is not automatically managed
    * memory has to be released manually
    * "unlimited"  size
    * variables allocated are accessible by any fuction anywhere in the program
    * allocations are expensive
    * we should take care about memory fragmentation
    * pointers

#Smart Pointer
    * a wrapper on raw pointer
    * ensures memory is released when variable goes out of scope

#Demo
    * inspect the code of memory basics with gdb and see how stack and heap evolve
    * dgb ./path_exe
    * list sources
    * b [function_name] sets the breakpoint
    * r runs the program
    * bt n show n stacks of the frame
    * info locals shows local variables in the scope
    * info args show args of function
    * n step one line
    * x /d 0x55555559bad0 deref pointer and get the value
    