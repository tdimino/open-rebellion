
undefined4 FUN_006115f0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 1;
  iVar1 = FUN_00612bd0((*(uint *)(param_1 + 4) & 0xffff) + *(int *)(param_1 + 8) * 0x10000);
  if ((iVar1 != 0) || (iVar1 = FUN_00611440(param_1), iVar1 == 0)) {
    uVar2 = FUN_00611580(param_1);
  }
  FUN_00611230(0);
  return uVar2;
}

