
undefined4 FUN_00479520(int *param_1)

{
  uint in_EAX;
  int iVar1;
  
  if (param_1 != (int *)0x0) {
    iVar1 = (**(code **)(*param_1 + 0x18))();
    if (iVar1 == 3) {
      in_EAX = 0;
    }
    else {
      if (iVar1 == 4) {
        return 1;
      }
      in_EAX = iVar1 - 5;
      if (in_EAX == 0) {
        return 2;
      }
    }
  }
  return in_EAX & 0xffff0000;
}

