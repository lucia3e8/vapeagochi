MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 192K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and statically
   allocated RAM variables. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM); 