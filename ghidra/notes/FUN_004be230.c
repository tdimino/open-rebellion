
int * __fastcall FUN_004be230(int param_1)

{
  uint uVar1;
  bool bVar2;
  short sVar3;
  void *pvVar4;
  uint *puVar5;
  void *pvVar6;
  undefined4 *puVar7;
  int iVar8;
  int *piVar9;
  bool bVar10;
  undefined1 local_24 [4];
  undefined1 local_20 [4];
  int *local_1c;
  void *local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639ecc;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_1c = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xfffffff7;
  sVar3 = FUN_005f50e0(param_1 + 0x3c);
  bVar10 = sVar3 != 0;
  for (pvVar4 = (void *)thunk_FUN_005f5060(param_1 + 0x3c); pvVar4 != (void *)0x0;
      pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
    puVar5 = FUN_00403040(pvVar4,local_24);
    local_4._0_1_ = 1;
    pvVar6 = FUN_004f5940(*(void **)(param_1 + 100),puVar5);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (pvVar6 != (void *)0x0) {
      if ((*(uint *)((int)pvVar6 + 0x30) & 0x74000f1) == 0) {
        if (*(int *)((int)pvVar6 + 0x38) != *(int *)(param_1 + 0x38)) {
          bVar10 = false;
          pvVar6 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 2;
          local_18 = pvVar6;
          if (pvVar6 == (void *)0x0) {
            pvVar6 = (void *)0x0;
          }
          else {
            puVar7 = FUN_00403040(pvVar4,local_20);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,3);
            pvVar6 = FUN_004f5b10(pvVar6,puVar7,0);
          }
          local_4 = 2;
          if (bVar2) {
            bVar2 = false;
            FUN_00619730();
          }
          local_4._0_1_ = 0;
          if (pvVar6 != (void *)0x0) {
            FUN_004f57b0(local_14,pvVar6);
          }
        }
      }
      else {
        bVar10 = false;
      }
    }
  }
  bVar2 = false;
  for (pvVar4 = (void *)thunk_FUN_005f5060(param_1 + 0x44); pvVar4 != (void *)0x0;
      pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
    puVar5 = FUN_00403040(pvVar4,local_20);
    local_4._0_1_ = 4;
    pvVar6 = FUN_004f5940(*(void **)(param_1 + 100),puVar5);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (pvVar6 != (void *)0x0) {
      if ((*(uint *)((int)pvVar6 + 0x30) & 0x74000f1) == 0) {
        if (*(int *)((int)pvVar6 + 0x38) != *(int *)(param_1 + 0x38)) {
          bVar10 = false;
          pvVar6 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 5;
          local_18 = pvVar6;
          if (pvVar6 == (void *)0x0) {
            pvVar6 = (void *)0x0;
          }
          else {
            puVar7 = FUN_00403040(pvVar4,local_24);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,6);
            pvVar6 = FUN_004f5b10(pvVar6,puVar7,0);
          }
          local_4 = 5;
          if (bVar2) {
            bVar2 = false;
            FUN_00619730();
          }
          local_4._0_1_ = 0;
          if (pvVar6 != (void *)0x0) {
            FUN_004f57b0(local_14,pvVar6);
          }
        }
      }
      else {
        bVar10 = false;
      }
    }
  }
  iVar8 = thunk_FUN_005f5060((int)local_14);
  piVar9 = local_1c;
  if ((iVar8 != 0) && (piVar9 = (int *)FUN_004f5cd0(0x201), piVar9 != (int *)0x0)) {
    piVar9[8] = *(int *)(param_1 + 0x24);
    (**(code **)(*piVar9 + 0x24))(local_14);
    (**(code **)(*piVar9 + 0x2c))(param_1 + 0x38);
  }
  if (((bVar10) && (uVar1 = *(uint *)(param_1 + 0x20), (uVar1 & 1) != 0)) && ((uVar1 & 2) != 0)) {
    *(uint *)(param_1 + 0x20) = uVar1 | 8;
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return piVar9;
}

