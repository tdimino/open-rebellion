
uint __fastcall FUN_00502020(int *param_1)

{
  uint uVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  uint *puVar5;
  int iVar6;
  int iVar7;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  uint local_6c;
  int local_68;
  undefined4 local_64;
  undefined4 local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006409b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar3 = thunk_FUN_005581b0(param_1);
  FUN_004fcd00(local_4c,param_1,4);
  local_4 = 0;
  FUN_00513120((int)local_4c);
  while (local_30 != 0) {
    if (uVar3 == 0) goto LAB_00502263;
    puVar5 = &local_6c;
    pvVar4 = (void *)FUN_0052bed0((int)local_4c);
    puVar5 = FUN_004025b0(pvVar4,puVar5);
    uVar3 = *puVar5 >> 0x18;
    FUN_00619730();
    local_54 = 0x30;
    local_50 = 0x40;
    local_4 = CONCAT31(local_4._1_3_,1);
    if ((uVar3 < 0x30) || (0x3f < uVar3)) {
      local_64 = 0x1c;
      local_60 = 0x20;
      if ((uVar3 < 0x1c) || (0x1f < uVar3)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) goto LAB_00502118;
      local_5c = 0x10;
      local_58 = 0x14;
      if ((uVar3 < 0x10) || (0x13 < uVar3)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) goto LAB_00502118;
      uVar3 = 0;
    }
    else {
LAB_00502118:
      uVar3 = 1;
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    if (uVar3 != 0) {
      iVar6 = FUN_0052bed0((int)local_4c);
      uVar3 = (uint)((((byte)*(undefined4 *)(iVar6 + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0);
    }
    FUN_005130d0((int)local_4c);
  }
  if (uVar3 != 0) {
    local_6c = 0;
    uVar3 = FUN_005007a0(param_1,&local_6c);
    if ((uVar3 != 0) && (uVar3 = (uint)((int)local_6c < 2), uVar3 != 0)) {
      iVar6 = param_1[0x18];
      if ((iVar6 < 0) || (iVar7 = (**(code **)(*param_1 + 0x248))(), iVar7 < iVar6)) {
        uVar3 = 0;
      }
      else {
        uVar3 = 1;
      }
      if (uVar3 != 0) {
        iVar6 = param_1[0x18];
        iVar7 = (**(code **)(*param_1 + 0x248))();
        if (iVar6 == iVar7) {
          uVar3 = (uint)param_1[0x14] >> 3 & 1;
        }
        if (uVar3 != 0) {
          iVar6 = FUN_00500670();
          uVar3 = (uint)(iVar6 != 0);
          if (uVar3 != 0) {
            iVar6 = FUN_00500670();
            uVar3 = (uint)(iVar6 != 0);
            if (uVar3 != 0) {
              iVar6 = FUN_00500670();
              uVar3 = (uint)(iVar6 != 0);
              if (uVar3 != 0) {
                iVar6 = FUN_00500670();
                uVar3 = (uint)(iVar6 != 0);
                if (uVar3 != 0) {
                  bVar2 = FUN_00500680(param_1);
                  uVar3 = (uint)(CONCAT31(extraout_var,bVar2) != 0);
                  if (uVar3 != 0) {
                    iVar6 = (**(code **)(*param_1 + 0x248))();
                    uVar3 = (uint)(iVar6 != 0);
                  }
                }
              }
            }
          }
        }
      }
    }
  }
LAB_00502263:
  local_68 = 0;
  if (uVar3 != 0) {
    bVar2 = FUN_004f6b50(param_1,&local_68);
    uVar3 = CONCAT31(extraout_var_00,bVar2);
    if (uVar3 != 0) {
      uVar1 = param_1[0x14];
      FUN_004fcd00(auStack_2c,param_1,4);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00513120((int)auStack_2c);
      while ((uVar1 = uVar1 >> 0xe & 1, iStack_10 != 0 && (uVar1 == 0))) {
        iVar6 = FUN_0052bed0((int)auStack_2c);
        uVar1 = *(uint *)(iVar6 + 0x50);
        FUN_005130d0((int)auStack_2c);
      }
      if (((*(byte *)(param_1 + 0x14) & 8) == 0) || (uVar1 == 0)) {
        uVar3 = ~*(uint *)(local_68 + 0x50) >> 3 & 1;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_004fcd80(auStack_2c);
      if (uVar3 != 0) {
        iVar6 = (**(code **)(*param_1 + 0x26c))();
        pvVar4 = FUN_005039d0(auStack_2c,param_1,3);
        local_4._0_1_ = 3;
        iVar7 = FUN_00513180((int)pvVar4);
        local_4._0_1_ = 0;
        FUN_00503ad0(auStack_2c);
        uVar3 = (uint)(iVar7 <= iVar6);
        if (uVar3 != 0) {
          iVar6 = (**(code **)(*param_1 + 0x270))();
          pvVar4 = FUN_00504c40(auStack_2c,param_1,3);
          local_4._0_1_ = 4;
          iVar7 = FUN_00513180((int)pvVar4);
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00504d40(auStack_2c);
          uVar3 = (uint)(iVar7 <= iVar6);
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004fcd80(local_4c);
  ExceptionList = pvStack_c;
  return uVar3;
}

