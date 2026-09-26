
undefined4 __fastcall FUN_00402230(undefined1 *param_1)

{
  int *piVar1;
  short sVar2;
  int *piVar3;
  uint uVar4;
  undefined4 uVar5;
  uint *puVar6;
  int iVar7;
  uint extraout_ECX;
  uint extraout_ECX_00;
  uint extraout_ECX_01;
  uint extraout_ECX_02;
  uint uVar8;
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00629258;
  local_c = ExceptionList;
  if ((*(uint *)(param_1 + 0x30) & 0x80000000) == 0) {
    return 0;
  }
  piVar1 = (int *)(param_1 + 0x40);
  ExceptionList = &local_c;
  *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) & 0x7080440e;
  piVar3 = piVar1;
  for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
    *piVar3 = 0;
    piVar3 = piVar3 + 1;
  }
  local_10 = param_1;
  piVar3 = FUN_004f3110(*(int *)(param_1 + 0x1c),(uint *)(param_1 + 0x18));
  if (piVar3 == (int *)0x0) {
    ExceptionList = local_c;
    return 1;
  }
  if ((*(byte *)(piVar3 + 0x14) & 8) != 0) {
    ExceptionList = local_c;
    return 1;
  }
  *(uint *)(param_1 + 0x20) = (uint)piVar3[9] >> 6 & 3;
  if ((*(byte *)(piVar3 + 0x1e) & 0x80) != 0) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x80;
  }
  if ((piVar3[0x1e] & 0x200U) != 0) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x400000;
  }
  if ((*(byte *)(piVar3 + 0x14) & 0x10) != 0) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x20;
  }
  if ((*(byte *)(piVar3 + 0x14) & 4) == 0) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 1;
  }
  uVar8 = *(uint *)(param_1 + 0x30);
  if ((uVar8 & 0x20000000) == 0) {
    uVar5 = FUN_004f2990((int)piVar3);
    *(undefined4 *)(param_1 + 100) = uVar5;
    goto LAB_004023f8;
  }
  if ((*(byte *)(piVar3 + 0x2b) & 1) != 0) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x40;
  }
  if ((short)piVar3[0x25] != 0) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x10;
  }
  if (((*(uint *)(param_1 + 0x34) & 0x800) != 0) &&
     (uVar4 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x3c) + 4)), uVar8 = extraout_ECX,
     (uVar4 & 0x2000000) == 0)) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x8000000;
  }
  if (((*(uint *)(param_1 + 0x34) & 0x1000) != 0) &&
     (uVar4 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x3c) + 4)), uVar8 = extraout_ECX_00,
     (uVar4 & 0x4000000) == 0)) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x8000000;
  }
  if (((*(uint *)(param_1 + 0x34) & 0x2000) != 0) &&
     (uVar4 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x3c) + 4)), uVar8 = extraout_ECX_01,
     (uVar4 & 0x8000000) == 0)) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x8000000;
  }
  sVar2 = *(short *)((int)piVar3 + 0x96);
  if (sVar2 == 1) {
    uVar4 = *(uint *)(param_1 + 0x30) | 0x4000000;
LAB_0040238a:
    *(uint *)(param_1 + 0x30) = uVar4;
  }
  else {
    if (sVar2 == 2) {
      uVar4 = *(uint *)(param_1 + 0x30) | 0x1000000;
      goto LAB_0040238a;
    }
    if (sVar2 == 3) {
      uVar4 = *(uint *)(param_1 + 0x30) | 0x2000000;
      goto LAB_0040238a;
    }
  }
  iVar7 = (int)(short)piVar3[0x23];
  *(int *)(param_1 + 0x60) = iVar7;
  if ((0 < iVar7) && (iVar7 < 100)) {
    local_10 = &stack0xffffffdc;
    FUN_00403040(param_1,&stack0xffffffdc);
    iVar7 = FUN_00402cf0(uVar8);
    uVar8 = extraout_ECX_02;
    if (iVar7 == 0) {
      *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x100;
    }
  }
  if (99 < *(int *)(param_1 + 0x60)) {
    local_10 = &stack0xffffffdc;
    FUN_00403040(param_1,&stack0xffffffdc);
    iVar7 = FUN_00402cf0(uVar8);
    if (iVar7 != 0) {
      *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x200;
    }
  }
LAB_004023f8:
  iVar7 = (**(code **)(*piVar3 + 0x1dc))();
  *piVar1 = iVar7;
  uVar5 = (**(code **)(*piVar3 + 0x1e0))();
  *(undefined4 *)(param_1 + 0x44) = uVar5;
  uVar5 = (**(code **)(*piVar3 + 0x1e4))();
  *(undefined4 *)(param_1 + 0x48) = uVar5;
  uVar5 = (**(code **)(*piVar3 + 0x1e8))();
  *(undefined4 *)(param_1 + 0x4c) = uVar5;
  uVar5 = (**(code **)(*piVar3 + 0x1ec))();
  *(undefined4 *)(param_1 + 0x50) = uVar5;
  uVar5 = (**(code **)(*piVar3 + 0x1f0))();
  *(undefined4 *)(param_1 + 0x54) = uVar5;
  uVar5 = (**(code **)(*piVar3 + 500))();
  *(undefined4 *)(param_1 + 0x58) = uVar5;
  uVar5 = (**(code **)(*piVar3 + 0x1f8))();
  *(undefined4 *)(param_1 + 0x5c) = uVar5;
  if (((param_1[0x34] & 1) != 0) && (0x4f < *piVar1)) {
    *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x8000000;
  }
  if (((*(uint *)(param_1 + 0x38) & 0xff000000) == 0) && ((void *)piVar3[7] != (void *)0x0)) {
    puVar6 = FUN_004025b0((void *)piVar3[7],(uint *)&local_10);
    uStack_4 = 0;
    FUN_004f26d0(param_1 + 0x38,puVar6);
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return 1;
}

