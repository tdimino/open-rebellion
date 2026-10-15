
undefined4 __fastcall FUN_004b16d0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  iVar1 = *(int *)(*(int *)(param_1 + 0x10) + 0xd0);
  if ((1 < iVar1) || ((*(int *)(*(int *)(param_1 + 0x10) + 0x98) == 0 && (0 < iVar1)))) {
    uVar2 = 1;
  }
  return uVar2;
}

