
void __fastcall FUN_004b8b70(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  void *pvVar4;
  int iVar5;
  uint uVar6;
  
  FUN_004b8f80(param_1);
  iVar3 = FUN_0047aa90(*(void **)(param_1 + 0xc),(undefined4 *)(param_1 + 0x5c));
  pvVar4 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                        (uint *)(*(int *)(param_1 + 0x10) + 0x34));
  if (pvVar4 != (void *)0x0) {
    iVar1 = *(int *)(param_1 + 0xc);
    iVar5 = (*(int *)((int)pvVar4 + 400) * 3) / 2;
    if (iVar5 < *(int *)(iVar1 + 0x164)) {
      iVar5 = *(int *)(iVar1 + 0x164);
    }
    if (*(int *)(param_1 + 0x34) < iVar5) {
      *(int *)(param_1 + 0x34) = iVar5;
    }
    iVar5 = (*(int *)((int)pvVar4 + 0x194) * 3) / 2;
    if (iVar3 != 0) {
      iVar5 = iVar5 + (*(int *)(iVar3 + 0xb4) * 3) / 2;
    }
    if (iVar5 < *(int *)(iVar1 + 0x168)) {
      iVar5 = *(int *)(iVar1 + 0x168);
    }
    if (*(int *)(param_1 + 0x38) < iVar5) {
      *(int *)(param_1 + 0x38) = iVar5;
    }
    iVar5 = ((*(int *)((int)pvVar4 + 0x198) + *(int *)((int)pvVar4 + 0x138)) * 0xb) / 10;
    if (iVar5 < *(int *)(iVar1 + 0x16c)) {
      iVar5 = *(int *)(iVar1 + 0x16c);
    }
    if (*(int *)(param_1 + 0x40) < iVar5) {
      *(int *)(param_1 + 0x40) = iVar5;
    }
    if (iVar3 != 0) {
      iVar5 = *(int *)(iVar3 + 0x118);
    }
    if (*(int *)(param_1 + 0x3c) < iVar5) {
      *(int *)(param_1 + 0x3c) = iVar5;
    }
    uVar6 = FUN_005f4960((undefined4 *)(iVar1 + 4));
    if (((uVar6 & 4) != 0) && (0 < *(int *)((int)pvVar4 + 0x19c))) {
      *(undefined4 *)(param_1 + 0x44) = 1;
    }
    uVar6 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0xc) + 4));
    if ((uVar6 & 0x1000000) != 0) {
      *(undefined4 *)(param_1 + 0x4c) = 1;
    }
    *(undefined4 *)(param_1 + 0x50) = *(undefined4 *)(*(int *)(param_1 + 0x10) + 0x94);
    uVar2 = *(undefined4 *)(*(int *)(param_1 + 0x10) + 0x98);
    *(undefined4 *)(param_1 + 0x58) = 1;
    *(undefined4 *)(param_1 + 0x54) = uVar2;
  }
  return;
}

