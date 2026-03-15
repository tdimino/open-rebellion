
undefined4 __fastcall FUN_004d0590(int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  
  uVar4 = 0;
  iVar1 = FUN_004d1d90(param_1);
  if (iVar1 < 1) {
    if (iVar1 < 0) {
      uVar4 = 2;
    }
  }
  else {
    iVar2 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 100),(void *)0x29,
                         *(uint *)(param_1 + 0x50),*(int *)(param_1 + 0x54),1);
    if ((((*(uint *)(param_1 + 100) & 0xff000000) != 0) && (iVar2 != 0)) &&
       (iVar3 = iVar2 * *(int *)(param_1 + 0x48), iVar3 - iVar1 != 0 && iVar1 <= iVar3)) {
      *(int *)(param_1 + 0x48) = iVar1 / iVar2;
    }
    if (0 < *(int *)(param_1 + 0x48)) {
      FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0x2c) + 0x18));
      return 0xb;
    }
  }
  return uVar4;
}

