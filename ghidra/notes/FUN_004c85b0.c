
undefined4 __fastcall FUN_004c85b0(int param_1)

{
  uint uVar1;
  int *piVar2;
  int iVar3;
  undefined4 uVar4;
  
  uVar4 = 0;
  uVar1 = *(uint *)(*(int *)(param_1 + 0x2c) + 0x354);
  if (uVar1 != 0) {
    FUN_0042ef90((void *)(*(int *)(param_1 + 0x2c) + 0xec),uVar1);
    *(undefined4 *)(*(int *)(param_1 + 0x2c) + 0x354) = 0;
  }
  uVar1 = *(uint *)(*(int *)(param_1 + 0x2c) + 0x358);
  if (uVar1 != 0) {
    uVar1 = FUN_0042ef50((void *)(*(int *)(param_1 + 0x2c) + 0xec),uVar1);
    piVar2 = (int *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xec),uVar1);
    *(undefined4 *)(*(int *)(param_1 + 0x2c) + 0x358) = 0;
    if (piVar2 != (int *)0x0) {
      iVar3 = (**(code **)(*piVar2 + 0xc))();
      if (iVar3 == 2) {
        FUN_0041b5d0(*(void **)(param_1 + 0x2c),piVar2);
        uVar4 = 1;
      }
    }
  }
  return uVar4;
}

