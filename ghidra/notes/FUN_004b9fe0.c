
void __fastcall FUN_004b9fe0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  void *pvVar3;
  int iVar4;
  uint uVar5;
  
  pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                        (uint *)(*(int *)(param_1 + 0x10) + 0x34));
  if (pvVar3 != (void *)0x0) {
    iVar1 = *(int *)(param_1 + 0xc);
    iVar4 = (*(int *)((int)pvVar3 + 400) * 3) / 2;
    if (iVar4 < *(int *)(iVar1 + 0x164)) {
      iVar4 = *(int *)(iVar1 + 0x164);
    }
    if (*(int *)(param_1 + 0x34) < iVar4) {
      *(int *)(param_1 + 0x34) = iVar4;
    }
    iVar4 = (*(int *)((int)pvVar3 + 0x13c) * 3) / 2 + (*(int *)((int)pvVar3 + 0x194) * 3) / 2;
    if (iVar4 < *(int *)(iVar1 + 0x168)) {
      iVar4 = *(int *)(iVar1 + 0x168);
    }
    if (*(int *)(param_1 + 0x38) < iVar4) {
      *(int *)(param_1 + 0x38) = iVar4;
    }
    iVar4 = ((*(int *)((int)pvVar3 + 0x138) + *(int *)((int)pvVar3 + 0x198)) * 3) / 2;
    if (iVar4 < *(int *)(iVar1 + 0x16c)) {
      iVar4 = *(int *)(iVar1 + 0x16c);
    }
    if (*(int *)(param_1 + 0x40) < iVar4) {
      *(int *)(param_1 + 0x40) = iVar4;
    }
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x44) = 0;
    uVar5 = FUN_005f4960((undefined4 *)(iVar1 + 4));
    if ((uVar5 & 0x1000000) != 0) {
      *(undefined4 *)(param_1 + 0x4c) = 1;
    }
    *(undefined4 *)(param_1 + 0x50) = 0;
    uVar2 = *(undefined4 *)(*(int *)(param_1 + 0x10) + 0x98);
    *(undefined4 *)(param_1 + 0x58) = 1;
    *(undefined4 *)(param_1 + 0x54) = uVar2;
  }
  return;
}

