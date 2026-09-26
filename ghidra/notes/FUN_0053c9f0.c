
undefined4 FUN_0053c9f0(void)

{
  int *unaff_ESI;
  void *in_stack_0000001c;
  undefined4 in_stack_00000024;
  undefined4 in_stack_00000030;
  
  if ((undefined4 *)*unaff_ESI != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)*unaff_ESI)();
    *unaff_ESI = 0;
  }
  in_stack_00000024 = 0xffffffff;
  FUN_004f4380((undefined4 *)&stack0x0000000c);
  ExceptionList = in_stack_0000001c;
  return in_stack_00000030;
}

