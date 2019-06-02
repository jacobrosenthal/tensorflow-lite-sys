MEMORY
{
  /* STM32F103 "Blue Pill". Should be 64K of flash, is actually 128K. */
  /* https://wiki.stm32duino.com/index.php?title=Blue_Pill */
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}