
void __fastcall FUN_0048df20(int param_1)

{
  int *piVar1;
  void *pvVar2;
  int iVar3;
  bool bVar4;
  ushort uVar5;
  int *piVar6;
  int *piVar7;
  int *piVar8;
  uint uVar9;
  
  piVar6 = FUN_004f2ec0(*(int *)(param_1 + 0x20),(uint *)(param_1 + 0x68));
  piVar7 = FUN_004f2ec0(*(int *)(param_1 + 0x20),(uint *)(param_1 + 0x6c));
  bVar4 = false;
  if ((piVar6 == (int *)0x0) || (piVar7 == (int *)0x0)) {
    *(undefined4 *)(param_1 + 0x40) = 0;
    return;
  }
  piVar1 = piVar7 + 0xc;
  piVar8 = piVar6 + 0xc;
  FUN_0060b9d0((void *)(param_1 + 0x44),0x71a8,piVar8,piVar1,(undefined4 *)0x0,(undefined4 *)0x0);
  pvVar2 = (void *)(param_1 + 0x50);
  FUN_0060b9d0(pvVar2,0x71a9,piVar8,piVar1,(undefined4 *)0x0,(undefined4 *)0x0);
  if ((*(byte *)(piVar6 + 0x2b) & 1) == 0) {
    if ((short)piVar6[0x25] != 0) {
      uVar9 = 0x71ab;
      goto LAB_0048dfdc;
    }
  }
  else {
    uVar9 = 0x71aa;
LAB_0048dfdc:
    bVar4 = true;
    FUN_0060b9d0(pvVar2,uVar9,piVar8,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
  }
  if ((*(byte *)(piVar7 + 0x2b) & 1) == 0) {
    if ((short)piVar7[0x25] == 0) goto LAB_0048e02f;
    uVar9 = 0x71ad;
  }
  else {
    uVar9 = 0x71ac;
  }
  bVar4 = true;
  FUN_0060b9d0(pvVar2,uVar9,piVar1,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
LAB_0048e02f:
  if (!bVar4) {
    FUN_0060b9d0(pvVar2,0x71ae,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
  }
  iVar3 = *(int *)(param_1 + 0x68);
  FUN_00619730();
  if (iVar3 != 0x32000242) {
    iVar3 = *(int *)(param_1 + 0x6c);
    FUN_00619730();
    if (iVar3 == 0x35000281) {
      *(undefined2 *)(param_1 + 0x2e) = 0x41f;
      *(undefined2 *)(param_1 + 0x30) = 0x46a;
      uVar5 = FUN_004c4990(&DAT_00000137);
      *(ushort *)(param_1 + 0x32) = uVar5;
      return;
    }
    *(undefined2 *)(param_1 + 0x2e) = 0x420;
    *(undefined2 *)(param_1 + 0x30) = 0x46b;
    uVar5 = FUN_004c4990(&DAT_00000138);
    *(ushort *)(param_1 + 0x32) = uVar5;
    return;
  }
  if ((*(byte *)(piVar6 + 0x2c) & 8) == 0) {
    iVar3 = *(int *)(param_1 + 0x6c);
    FUN_00619730();
    if (iVar3 == 0x35000281) {
      *(undefined2 *)(param_1 + 0x2e) = 0x423;
      *(undefined2 *)(param_1 + 0x30) = 0x46e;
      uVar5 = FUN_004c4990(&DAT_00000118);
      *(ushort *)(param_1 + 0x32) = uVar5;
      return;
    }
    *(undefined2 *)(param_1 + 0x2e) = 0x425;
    *(undefined2 *)(param_1 + 0x30) = 0x470;
    uVar5 = FUN_004c4990(&DAT_00000119);
    *(ushort *)(param_1 + 0x32) = uVar5;
    return;
  }
  iVar3 = *(int *)(param_1 + 0x6c);
  FUN_00619730();
  if (iVar3 == 0x35000281) {
    *(undefined2 *)(param_1 + 0x2e) = 0x424;
    *(undefined2 *)(param_1 + 0x30) = 0x46f;
    uVar5 = FUN_004c4990(&DAT_0000011a);
    *(ushort *)(param_1 + 0x32) = uVar5;
    return;
  }
  *(undefined2 *)(param_1 + 0x2e) = 0x426;
  *(undefined2 *)(param_1 + 0x30) = 0x471;
  uVar5 = FUN_004c4990(&DAT_0000011b);
  *(ushort *)(param_1 + 0x32) = uVar5;
  return;
}

