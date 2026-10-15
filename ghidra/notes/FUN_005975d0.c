
undefined2 FUN_005975d0(short param_1)

{
  int iVar1;
  
  iVar1 = FUN_005f5060();
  if (iVar1 == 0) {
    return 0;
  }
  do {
    if (*(short *)(iVar1 + 0x18) == param_1) {
      return *(undefined2 *)(iVar1 + 0x1a);
    }
    iVar1 = *(int *)(iVar1 + 0x10);
  } while (iVar1 != 0);
  return 0;
}

