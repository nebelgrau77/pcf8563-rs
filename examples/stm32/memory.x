/* Linker script for the STM32L432 */
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x8000000, LENGTH = 256K 
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* NOTE: Do *NOT* modify `_stack_start` unless you know what you are doing. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
