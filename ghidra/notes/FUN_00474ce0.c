
int * __fastcall FUN_00474ce0(int param_1)

{
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int *piVar6;
  uint *puVar7;
  int iVar8;
  uint local_6c;
  int local_68;
  undefined4 local_64;
  int *local_60;
  undefined4 local_5c;
  uint local_58;
  undefined1 *local_54;
  void *local_50;
  undefined4 local_4c [2];
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c [4];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632b9a;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_5c = 0;
  local_60 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_68 = param_1;
  FUN_004355d0(local_3c);
  local_4 = 0;
  FUN_004ece30(&local_64);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_6c);
  local_4._0_1_ = 2;
  FUN_004f4340(local_4c);
  iVar8 = *(int *)(param_1 + 0x4c);
  local_54 = &stack0xffffff80;
  puVar5 = (undefined4 *)(param_1 + 0x2c);
  local_4._0_1_ = 3;
  FUN_004f26d0(&stack0xffffff80,puVar5);
  pvVar3 = FUN_00419e40((void *)(iVar8 + 0x78));
  if ((pvVar3 != (void *)0x0) && (0 < *(int *)((int)pvVar3 + 0x80))) {
    iVar8 = *(int *)(param_1 + 0x4c);
    local_54 = &stack0xffffff80;
    FUN_004f26d0(&stack0xffffff80,(undefined4 *)((int)pvVar3 + 0x28));
    pvVar4 = FUN_00403d30((void *)(iVar8 + 0x2c));
    param_1 = local_68;
    if ((pvVar4 != (void *)0x0) && ((*(byte *)((int)pvVar4 + 0x30) & 1) != 0)) {
      FUN_004f26d0(&local_64,(undefined4 *)((int)pvVar3 + 0x28));
      iVar8 = local_68;
      pvVar3 = FUN_0041ac80(*(void **)(local_68 + 0x4c),puVar5,0x4000,0x20800,9,(void *)0x1);
      FUN_00435790(local_3c,(int)pvVar3);
      puVar5 = FUN_004357b0(local_3c,&local_5c);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_6c,puVar5);
      local_4._0_1_ = 3;
      FUN_00619730();
      while( true ) {
        local_44 = 0x14;
        local_40 = 0x1c;
        if ((local_6c >> 0x18 < 0x14) || (0x1b < local_6c >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) break;
        piVar6 = FUN_004f2e20(*(int *)(iVar8 + 0x20),&local_6c);
        if (piVar6 != (int *)0x0) {
          FUN_00504c40(local_2c,piVar6,1);
          local_4 = CONCAT31(local_4._1_3_,5);
          FUN_00513120((int)local_2c);
          while (local_10 != 0) {
            pvVar3 = (void *)FUN_00618b70(0x20);
            local_4._0_1_ = 6;
            local_50 = pvVar3;
            if (pvVar3 == (void *)0x0) {
              pvVar3 = (void *)0x0;
            }
            else {
              puVar7 = &local_58;
              pvVar4 = (void *)FUN_0052bed0((int)local_2c);
              puVar7 = FUN_004025b0(pvVar4,puVar7);
              bVar2 = true;
              local_5c = 1;
              local_4 = CONCAT31(local_4._1_3_,7);
              pvVar3 = FUN_004f5b10(pvVar3,puVar7,0);
            }
            local_4 = 6;
            if (bVar2) {
              bVar2 = false;
              FUN_00619730();
            }
            local_4 = CONCAT31(local_4._1_3_,5);
            if (pvVar3 != (void *)0x0) {
              FUN_004f57b0(local_4c,pvVar3);
            }
            FUN_005130d0((int)local_2c);
          }
          local_4._1_3_ = (uint3)((uint)local_4 >> 8);
          local_4._0_1_ = 3;
          FUN_00504d40(local_2c);
        }
        puVar5 = FUN_004357b0(local_3c,&local_54);
        local_4._0_1_ = 8;
        FUN_004f26d0(&local_6c,puVar5);
        local_4._0_1_ = 3;
        FUN_00619730();
      }
      FUN_005f58b0(local_3c);
      param_1 = local_68;
    }
  }
  iVar8 = thunk_FUN_005f5060((int)local_4c);
  if ((iVar8 != 0) && (piVar6 = (int *)FUN_004f5cd0(0x201), local_60 = piVar6, piVar6 != (int *)0x0)
     ) {
    piVar6[8] = *(int *)(param_1 + 0x20);
    (**(code **)(*piVar6 + 0x24))();
    (**(code **)(*piVar6 + 0x2c))(&local_68);
  }
  local_4._0_1_ = 2;
  FUN_004f4380(local_4c);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00435610(local_3c);
  ExceptionList = pvStack_c;
  return local_60;
}

