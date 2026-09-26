
undefined4 __fastcall FUN_004cd230(int param_1)

{
  int iVar1;
  int iVar2;
  int *this;
  int iVar3;
  undefined4 uVar4;
  
  uVar4 = 0;
  iVar1 = FUN_004cec20(param_1);
  if (iVar1 < 1) {
    if (iVar1 < 0) {
      uVar4 = 5;
    }
  }
  else {
    iVar2 = FUN_0041cd80(10);
    iVar2 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 100),(void *)0x28,0x200000,
                         ((5 < iVar2) - 1 & 0xffff8400) + 0x8000,1);
    if ((((*(uint *)(param_1 + 100) & 0xff000000) != 0) && (iVar2 != 0)) &&
       (iVar3 = iVar2 * *(int *)(param_1 + 0x68), iVar3 - iVar1 != 0 && iVar1 <= iVar3)) {
      *(int *)(param_1 + 0x68) = iVar1 / iVar2;
    }
    if (0 < *(int *)(param_1 + 0x68)) {
      this = FUN_004f2f60(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x58));
      if (this != (int *)0x0) {
        iVar1 = FUN_00528040(this,0);
        if (iVar1 < *(int *)(param_1 + 0x68)) {
          *(int *)(param_1 + 0x68) = iVar1;
        }
      }
      FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0x2c) + 0x20));
      return 9;
    }
  }
  return uVar4;
}

