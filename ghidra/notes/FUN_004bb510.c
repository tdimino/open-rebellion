
void __fastcall FUN_004bb510(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  void *pvVar3;
  uint uVar4;
  int iVar5;
  
  pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                        (uint *)(*(int *)(param_1 + 0x10) + 0x34));
  if (pvVar3 != (void *)0x0) {
    iVar1 = (*(int *)((int)pvVar3 + 400) * 0xc) / 10;
    if (*(int *)(param_1 + 0x34) < iVar1) {
      *(int *)(param_1 + 0x34) = iVar1;
    }
    iVar1 = *(int *)(param_1 + 0xc);
    iVar5 = (*(int *)((int)pvVar3 + 0x194) * 0xc) / 10;
    if (iVar5 < *(int *)(iVar1 + 0x168)) {
      iVar5 = *(int *)(iVar1 + 0x168);
    }
    if (*(int *)(param_1 + 0x38) < iVar5) {
      *(int *)(param_1 + 0x38) = iVar5;
    }
    iVar5 = ((*(int *)((int)pvVar3 + 0x138) + *(int *)((int)pvVar3 + 0x198)) * 0xc) / 10;
    if (*(int *)(param_1 + 0x40) < iVar5) {
      *(int *)(param_1 + 0x40) = iVar5;
    }
    *(undefined4 *)(param_1 + 0x3c) = 0;
    iVar5 = (*(int *)((int)pvVar3 + 0x144) * 0xb) / 10;
    if (iVar5 < *(int *)(iVar1 + 0x174)) {
      iVar5 = *(int *)(iVar1 + 0x174);
    }
    if (*(int *)(param_1 + 0x48) < iVar5) {
      *(int *)(param_1 + 0x48) = iVar5;
    }
    *(undefined4 *)(param_1 + 0x44) = 0;
    uVar4 = FUN_005f4960((undefined4 *)(iVar1 + 4));
    if ((uVar4 & 0x1000000) != 0) {
      *(undefined4 *)(param_1 + 0x4c) = 1;
    }
    *(undefined4 *)(param_1 + 0x50) = 0;
    uVar2 = *(undefined4 *)(*(int *)(param_1 + 0x10) + 0x98);
    *(undefined4 *)(param_1 + 0x58) = 1;
    *(undefined4 *)(param_1 + 0x54) = uVar2;
  }
  return;
}

