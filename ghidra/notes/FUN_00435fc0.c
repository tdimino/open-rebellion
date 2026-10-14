
undefined4 __fastcall FUN_00435fc0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  if (*(int *)(param_1 + 0xc) != 0) {
    iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x24);
    if (iVar1 != 1) {
      if (iVar1 != 2) {
        return uVar2;
      }
      return 2;
    }
    uVar2 = 1;
  }
  return uVar2;
}

