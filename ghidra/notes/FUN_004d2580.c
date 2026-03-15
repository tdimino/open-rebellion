
undefined4 __fastcall FUN_004d2580(int param_1)

{
  uint uVar1;
  int *piVar2;
  int iVar3;
  undefined4 uVar4;
  
  uVar4 = 0;
  uVar1 = *(uint *)(*(int *)(param_1 + 0x2c) + 0x314);
  if (uVar1 != 0) {
    FUN_0042f690((void *)(*(int *)(param_1 + 0x2c) + 0xa8),uVar1);
    *(undefined4 *)(*(int *)(param_1 + 0x2c) + 0x314) = 0;
  }
  iVar3 = *(int *)(param_1 + 0x2c);
  if (*(int *)(iVar3 + 0x318) != 0) {
    uVar1 = FUN_0042f640((void *)(iVar3 + 0xa8),*(int *)(iVar3 + 0x318),*(uint *)(iVar3 + 0x31c));
    piVar2 = (int *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xa8),uVar1);
    *(undefined4 *)(*(int *)(param_1 + 0x2c) + 0x318) = 0;
    if (piVar2 != (int *)0x0) {
      iVar3 = (**(code **)(*piVar2 + 0x10))();
      if (iVar3 == 2) {
        FUN_0041b570(*(void **)(param_1 + 0x2c),piVar2);
        uVar4 = 1;
      }
    }
  }
  return uVar4;
}

