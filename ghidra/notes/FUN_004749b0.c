
int * __fastcall FUN_004749b0(int param_1)

{
  bool bVar1;
  void *pvVar2;
  uint *puVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  int *piVar7;
  undefined4 local_2c;
  int *local_28;
  undefined1 local_24 [4];
  undefined4 local_20;
  undefined1 *local_1c;
  void *local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632b2a;
  pvStack_c = ExceptionList;
  bVar1 = false;
  local_20 = 0;
  local_28 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = 0;
  FUN_004ece30(&local_2c);
  iVar6 = *(int *)(param_1 + 0x4c);
  local_1c = &stack0xffffffc0;
  local_4._0_1_ = 1;
  FUN_004f26d0(&stack0xffffffc0,(undefined4 *)(param_1 + 0x2c));
  pvVar2 = FUN_00419e40((void *)(iVar6 + 0x78));
  if (((pvVar2 != (void *)0x0) && ((*(uint *)((int)pvVar2 + 0x38) & 0x3000) != 0)) &&
     ((*(uint *)((int)pvVar2 + 0x38) & 0xf0000802) == 0)) {
    FUN_004f26d0(&local_2c,(undefined4 *)((int)pvVar2 + 0x28));
    for (pvVar2 = (void *)thunk_FUN_005f5060((int)pvVar2 + 0x48); pvVar2 != (void *)0x0;
        pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
      puVar3 = FUN_00403040(pvVar2,local_24);
      local_4._0_1_ = 2;
      pvVar4 = FUN_004f5940((void *)(*(int *)(param_1 + 0x4c) + 0x58),puVar3);
      local_4._0_1_ = 1;
      FUN_00619730();
      if (((pvVar4 != (void *)0x0) && ((*(uint *)((int)pvVar4 + 0x24) & 0x801000) != 0)) &&
         ((*(uint *)((int)pvVar4 + 0x24) & 0x800) == 0)) {
        pvVar4 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 3;
        local_18 = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          puVar5 = FUN_00403040(pvVar2,&local_1c);
          bVar1 = true;
          local_20 = 1;
          local_4 = CONCAT31(local_4._1_3_,4);
          pvVar4 = FUN_004f5b10(pvVar4,puVar5,0);
        }
        local_4 = 3;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 1;
        if (pvVar4 != (void *)0x0) {
          FUN_004f57b0(local_14,pvVar4);
        }
      }
    }
  }
  iVar6 = thunk_FUN_005f5060((int)local_14);
  if ((iVar6 != 0) && (piVar7 = (int *)FUN_004f5cd0(0x270), local_28 = piVar7, piVar7 != (int *)0x0)
     ) {
    piVar7[8] = *(int *)(param_1 + 0x20);
    (**(code **)(*piVar7 + 0x24))();
    (**(code **)(*piVar7 + 0x2c))(&stack0xffffffd0);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return local_28;
}

