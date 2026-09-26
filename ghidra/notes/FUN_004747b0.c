
void __fastcall FUN_004747b0(void *param_1)

{
  int iVar1;
  bool bVar2;
  short sVar3;
  void *pvVar4;
  uint *puVar5;
  void *pvVar6;
  uint uVar7;
  uint local_30;
  int local_2c;
  undefined1 local_28 [4];
  undefined1 *local_24;
  undefined1 *local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632ad8;
  local_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &local_c;
  FUN_004ece30(&local_30);
  iVar1 = *(int *)((int)param_1 + 0x4c);
  local_24 = &stack0xffffffbc;
  local_4 = 0;
  FUN_004f26d0(&stack0xffffffbc,(undefined4 *)((int)param_1 + 0x2c));
  pvVar4 = FUN_00419e40((void *)(iVar1 + 0x78));
  if ((((pvVar4 != (void *)0x0) && ((*(uint *)((int)pvVar4 + 0x38) & 1) != 0)) &&
      ((*(uint *)((int)pvVar4 + 0x38) & 8) != 0)) &&
     (sVar3 = FUN_005f50e0((int)pvVar4 + 0x48), sVar3 != 0)) {
    bVar2 = true;
  }
  if (!bVar2) {
    FUN_00476080((int)param_1);
  }
  pvVar4 = (void *)thunk_FUN_005f5060((int)param_1 + 0x24);
  while (pvVar4 != (void *)0x0) {
    local_2c = 0;
    puVar5 = FUN_00403040(pvVar4,local_28);
    local_4._0_1_ = 1;
    pvVar6 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x4c) + 0x58),puVar5);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if ((pvVar6 != (void *)0x0) && ((*(uint *)((int)pvVar6 + 0x24) & 0x4000) != 0)) {
      FUN_004f26d0(&local_30,(undefined4 *)((int)pvVar6 + 0x2c));
      local_1c = 8;
      uVar7 = *(uint *)((int)param_1 + 0x2c) >> 0x18;
      local_18 = 0x10;
      if ((uVar7 < 8) || ((0xf < uVar7 || (*(uint *)((int)param_1 + 0x2c) != local_30)))) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        local_2c = 1;
      }
      else {
        local_14 = 8;
        uVar7 = *(uint *)((int)param_1 + 0x2c) >> 0x18;
        local_10 = 0x10;
        if ((uVar7 < 8) || (0xf < uVar7)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          iVar1 = *(int *)((int)param_1 + 0x4c);
          local_20 = &stack0xffffffbc;
          FUN_004f26d0(&stack0xffffffbc,&local_30);
          pvVar6 = FUN_00419e40((void *)(iVar1 + 0x78));
          if ((pvVar6 != (void *)0x0) && (local_2c = 1, (*(byte *)((int)pvVar6 + 0x38) & 4) != 0)) {
            FUN_00475fe0(param_1,&local_30);
          }
        }
      }
    }
    if (local_2c == 0) {
      pvVar6 = *(void **)((int)pvVar4 + 0x10);
      puVar5 = FUN_00403040(pvVar4,&local_24);
      local_4._0_1_ = 2;
      FUN_00475f60(param_1,puVar5);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      pvVar4 = pvVar6;
    }
    else {
      pvVar4 = *(void **)((int)pvVar4 + 0x10);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

