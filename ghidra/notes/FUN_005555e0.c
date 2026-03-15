
uint FUN_005555e0(uint *param_1,void *param_2)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  uint uVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  undefined4 uVar8;
  undefined4 local_ac;
  undefined4 local_a8;
  uint uStack_a4;
  uint uStack_a0;
  uint local_9c;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 auStack_8c [7];
  int iStack_70;
  undefined4 auStack_6c [7];
  int iStack_50;
  undefined4 auStack_4c [8];
  undefined4 auStack_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006494ca;
  pvStack_c = ExceptionList;
  local_9c = 1;
  local_98 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_9c);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar2 = (int *)FUN_00504dc0(param_1);
  uVar4 = (uint)(piVar2 != (int *)0x0);
  local_9c = uVar4;
  if (uVar4 != 0) {
    iVar3 = (**(code **)(*piVar2 + 0x38))();
    if ((iVar3 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
      uStack_a0 = 0x22;
      uStack_a4 = 0x20;
      local_4 = 1;
      uVar4 = (**(code **)(*piVar2 + 4))();
      if ((uVar4 < uStack_a4) || (uStack_a0 <= uVar4)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      uVar4 = local_9c;
      if (bVar1) {
        FUN_004ece30(&uStack_a4);
        local_4 = 2;
        (**(code **)(*piVar2 + 0xc))(&uStack_a4);
        uStack_94 = 0x90;
        uStack_90 = 0x98;
        if ((uStack_a4 >> 0x18 < 0x90) || (0x97 < uStack_a4 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (((bVar1) && (iVar3 = FUN_00505f60(&uStack_a4), iVar3 != 0)) &&
           ((*(byte *)(iVar3 + 0x88) & 0x20) != 0)) {
          iVar7 = 0;
          iVar6 = 0;
          FUN_004ffe70(auStack_8c,iVar3,1);
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00513120((int)auStack_8c);
          while (iStack_70 != 0) {
            uVar8 = 1;
            uVar5 = FUN_0052bed0((int)auStack_8c);
            FUN_00502db0(auStack_6c,uVar5,uVar8);
            local_4 = CONCAT31(local_4._1_3_,4);
            FUN_00513120((int)auStack_6c);
            while (iStack_50 != 0) {
              uVar8 = 1;
              iVar7 = iVar7 + 1;
              uVar5 = FUN_0052bed0((int)auStack_6c);
              FUN_005039d0(auStack_4c,uVar5,uVar8);
              local_4._0_1_ = 5;
              iVar3 = FUN_00513180((int)auStack_4c);
              iVar6 = iVar6 + iVar3;
              local_4 = CONCAT31(local_4._1_3_,4);
              FUN_00503ad0(auStack_4c);
              FUN_005130d0((int)auStack_6c);
            }
            local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
            local_4 = CONCAT31(local_4._1_3_,3);
            FUN_00502eb0(auStack_6c);
            FUN_005130d0((int)auStack_8c);
          }
          FUN_005039d0(auStack_2c,local_ac,1);
          local_4._0_1_ = 6;
          iVar3 = FUN_00513180((int)auStack_2c);
          iVar3 = FUN_0055a020(iVar7,iVar6 + iVar3);
          if (iVar3 < 0x1f) {
            local_ac = 0x90;
            local_a8 = 4;
            local_4._0_1_ = 7;
            FUN_00520580(param_2,&local_ac);
            local_4._0_1_ = 6;
            FUN_00619730();
          }
          local_4._0_1_ = 3;
          FUN_00503ad0(auStack_2c);
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_004fff70(auStack_8c);
        }
        local_4 = 0xffffffff;
        FUN_00619730();
        uVar4 = local_9c;
      }
    }
    if (uVar4 != 0) {
      ExceptionList = pvStack_c;
      return uVar4;
    }
  }
  local_ac = 1;
  local_a8 = 1;
  local_4 = 8;
  FUN_00520580(param_2,&local_ac);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar4;
}

