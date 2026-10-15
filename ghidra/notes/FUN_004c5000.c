
uint FUN_004c5000(int *param_1)

{
  uint in_EAX;
  uint uVar1;
  
  uVar1 = in_EAX & 0xffff0000;
  if (param_1 != (int *)0x0) {
    uVar1 = (**(code **)(*param_1 + 0x30))();
    uVar1 = (uVar1 & 0xfff) + 0x1000;
  }
  return uVar1;
}

