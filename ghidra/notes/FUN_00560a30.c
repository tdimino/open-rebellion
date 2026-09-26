
uint FUN_00560a30(void *param_1,void *param_2)

{
  uint uVar1;
  uint uVar2;
  bool bVar3;
  bool bVar4;
  undefined3 extraout_var;
  int iVar5;
  uint *puVar6;
  void *pvVar7;
  uint *puVar8;
  void *pvVar9;
  uint uVar10;
  int local_50;
  uint local_4c;
  uint local_48;
  undefined4 local_44;
  uint local_40;
  undefined4 local_3c;
  uint local_38;
  uint local_34;
  uint local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a1c8;
  local_c = ExceptionList;
  local_50 = 0;
  uVar10 = 0;
  ExceptionList = &local_c;
  if ((*(uint *)((int)param_1 + 0xac) >> 7 & 1) != 0) {
    ExceptionList = &local_c;
    bVar4 = FUN_004f6b50(param_1,&local_50);
    uVar10 = CONCAT31(extraout_var,bVar4);
  }
  if (uVar10 != 0) {
    FUN_004f2640(local_2c,local_50,1,*(uint *)((int)param_1 + 0x24) >> 6 & 3);
    local_4._0_1_ = 0;
    local_4._1_3_ = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar5 = FUN_0052bed0((int)local_2c);
      local_30 = *(uint *)(iVar5 + 0xac) & 1;
      if (local_30 == 0) {
        iVar5 = FUN_0052bed0((int)local_2c);
        local_30 = *(uint *)(iVar5 + 0x78) >> 7 & 1;
        if (local_30 == 0) {
          puVar6 = FUN_004025b0(param_1,&local_48);
          puVar8 = &local_4c;
          local_4._0_1_ = 1;
          pvVar7 = (void *)FUN_0052bed0((int)local_2c);
          puVar8 = FUN_004025b0(pvVar7,puVar8);
          uVar1 = *puVar8;
          uVar2 = *puVar6;
          FUN_00619730();
          local_4._0_1_ = 0;
          FUN_00619730();
          if (uVar1 != uVar2) {
            iVar5 = FUN_0052bed0((int)local_2c);
            local_30 = *(uint *)(iVar5 + 0xac) >> 5 & 1;
            if (local_30 != 0) {
              iVar5 = FUN_0052bed0((int)local_2c);
              local_30 = *(uint *)(iVar5 + 0xac) >> 4 & 1;
              if (local_30 == 0) {
                puVar8 = &local_40;
                bVar4 = false;
                pvVar7 = (void *)FUN_0052bed0((int)local_2c);
                puVar8 = FUN_004025b0(pvVar7,puVar8);
                local_44 = 0x31000241;
                uVar1 = *puVar8;
                local_4._0_1_ = 2;
                FUN_00619730();
                local_4._0_1_ = 0;
                FUN_00619730();
                if (uVar1 == 0x31000241) {
                  puVar8 = FUN_004025b0(param_1,&local_38);
                  local_4._0_1_ = 3;
                  local_3c = 0x32000242;
                  if ((*puVar8 == 0x32000242) && ((*(byte *)((int)param_1 + 0xb0) & 1) != 0)) {
                    bVar3 = true;
                  }
                  else {
                    bVar3 = false;
                  }
                  FUN_00619730();
                  local_4._0_1_ = 0;
                  FUN_00619730();
                  if (bVar3) {
                    bVar4 = true;
                    iVar5 = 1;
                    pvVar7 = param_2;
                    pvVar9 = (void *)FUN_0052bed0((int)local_2c);
                    iVar5 = FUN_0056fe90(pvVar9,iVar5,pvVar7);
                    if ((iVar5 == 0) || (uVar10 == 0)) {
                      uVar10 = 0;
                    }
                    else {
                      uVar10 = 1;
                    }
                  }
                }
                else {
                  bVar4 = true;
                }
                if (bVar4) {
                  iVar5 = 1;
                  pvVar7 = param_2;
                  pvVar9 = (void *)FUN_0052bed0((int)local_2c);
                  iVar5 = FUN_004ee950(pvVar9,iVar5,pvVar7);
                  if ((iVar5 == 0) || (uVar10 == 0)) {
                    bVar4 = false;
                  }
                  else {
                    bVar4 = true;
                  }
                  puVar8 = &local_34;
                  pvVar7 = (void *)FUN_0052bed0((int)local_2c);
                  puVar8 = FUN_004025b0(pvVar7,puVar8);
                  local_4._0_1_ = 4;
                  iVar5 = FUN_004ee680(param_1,(int *)puVar8,param_2);
                  if ((iVar5 == 0) || (!bVar4)) {
                    uVar10 = 0;
                  }
                  else {
                    uVar10 = 1;
                  }
                  local_4._0_1_ = 0;
                  FUN_00619730();
                }
              }
            }
          }
        }
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_004f26c0(local_2c);
  }
  ExceptionList = local_c;
  return uVar10;
}

