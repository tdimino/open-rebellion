
undefined4 __fastcall FUN_004df9a0(int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  undefined4 uVar5;
  
  uVar5 = 0;
  iVar1 = FUN_004e19c0(param_1);
  if (iVar1 < 1) {
    if (iVar1 < 0) {
      uVar5 = 8;
    }
  }
  else {
    iVar2 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 0x5c),(void *)0x29,
                         *(uint *)(param_1 + 0x6c),*(int *)(param_1 + 0x70),
                         *(undefined4 *)(param_1 + 0x74));
    if ((((*(uint *)(param_1 + 0x5c) & 0xff000000) != 0) && (iVar2 != 0)) &&
       (iVar3 = iVar2 * *(int *)(param_1 + 0x60), iVar3 - iVar1 != 0 && iVar1 <= iVar3)) {
      *(int *)(param_1 + 0x60) = iVar1 / iVar2;
    }
    if (0 < *(int *)(param_1 + 0x60)) {
      if (*(int *)(param_1 + 100) < *(int *)(param_1 + 0x60)) {
        *(int *)(param_1 + 0x60) = *(int *)(param_1 + 100);
      }
      FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0x2c) + 0x14));
      uVar5 = 0xc;
    }
  }
  uVar4 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x2c) + 4));
  if (((uVar4 & 0x400000) != 0) && (iVar1 < 1)) {
    if (9 < *(int *)((int)*(void **)(param_1 + 0x2c) + 900)) {
      FUN_00418270(*(void **)(param_1 + 0x2c),8);
    }
    if (3 < *(int *)((int)*(void **)(param_1 + 0x2c) + 0x380)) {
      FUN_00418270(*(void **)(param_1 + 0x2c),7);
    }
    if (0 < *(int *)((int)*(void **)(param_1 + 0x2c) + 0x37c)) {
      FUN_00418270(*(void **)(param_1 + 0x2c),6);
    }
  }
  return uVar5;
}

