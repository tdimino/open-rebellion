
void FUN_00472010(undefined4 param_1,void *param_2,void *param_3,undefined4 param_4)

{
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  uint *puVar4;
  int iVar5;
  uint uVar6;
  uint local_ac;
  uint local_a8;
  uint local_a4;
  uint local_a0;
  uint local_9c;
  uint local_98;
  undefined4 local_94;
  undefined4 local_90;
  undefined4 local_8c [7];
  int local_70;
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006327b8;
  pvStack_c = ExceptionList;
  uVar6 = 0;
  local_a8 = 0;
  ExceptionList = &pvStack_c;
  FUN_00502db0(local_2c,param_1,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    pvVar2 = (void *)FUN_0052bed0((int)local_2c);
    if (((*(uint *)((int)pvVar2 + 0x50) & 4) == 0) || ((*(uint *)((int)pvVar2 + 0x50) & 0x20) != 0))
    {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if (bVar1) {
      switch(param_4) {
      case 0x65:
        pvVar3 = (void *)FUN_00618b70(0x1c);
        local_4._0_1_ = 1;
        if (pvVar3 == (void *)0x0) {
          pvVar2 = (void *)0x0;
        }
        else {
          puVar4 = FUN_004025b0(pvVar2,&local_a0);
          uVar6 = uVar6 | 1;
          local_4 = CONCAT31(local_4._1_3_,2);
          local_a8 = uVar6;
          pvVar2 = FUN_004ecc70(pvVar3,puVar4);
        }
        local_4 = 1;
        if ((uVar6 & 1) != 0) {
          uVar6 = uVar6 & 0xfffffffe;
          FUN_00619730();
        }
        local_4 = local_4 & 0xffffff00;
        FUN_004f57b0(param_2,pvVar2);
        break;
      case 0x66:
        FUN_005039d0(local_4c,pvVar2,3);
        local_4 = CONCAT31(local_4._1_3_,3);
        FUN_00513120((int)local_4c);
        while (local_30 != 0) {
          iVar5 = FUN_0052bed0((int)local_4c);
          if ((*(uint *)(iVar5 + 0x50) >> 2 & 1) != 0) {
            pvVar2 = (void *)FUN_00618b70(0x1c);
            local_4._0_1_ = 4;
            if (pvVar2 == (void *)0x0) {
              pvVar2 = (void *)0x0;
            }
            else {
              puVar4 = &local_98;
              pvVar3 = (void *)FUN_0052bed0((int)local_4c);
              puVar4 = FUN_004025b0(pvVar3,puVar4);
              uVar6 = uVar6 | 2;
              local_4 = CONCAT31(local_4._1_3_,5);
              local_a8 = uVar6;
              pvVar2 = FUN_004ecc70(pvVar2,puVar4);
            }
            local_4 = 4;
            if ((uVar6 & 2) != 0) {
              uVar6 = uVar6 & 0xfffffffd;
              FUN_00619730();
            }
            local_4 = CONCAT31(local_4._1_3_,3);
            FUN_004f57b0(param_2,pvVar2);
          }
          FUN_005130d0((int)local_4c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_00503ad0(local_4c);
        break;
      case 0x67:
        FUN_00504c40(local_8c,pvVar2,3);
        local_4 = CONCAT31(local_4._1_3_,6);
        FUN_00513120((int)local_8c);
        while (local_70 != 0) {
          iVar5 = FUN_0052bed0((int)local_8c);
          if ((*(uint *)(iVar5 + 0x50) >> 2 & 1) != 0) {
            pvVar2 = (void *)FUN_00618b70(0x1c);
            local_4._0_1_ = 7;
            if (pvVar2 == (void *)0x0) {
              pvVar2 = (void *)0x0;
            }
            else {
              puVar4 = &local_9c;
              pvVar3 = (void *)FUN_0052bed0((int)local_8c);
              puVar4 = FUN_004025b0(pvVar3,puVar4);
              uVar6 = uVar6 | 4;
              local_4 = CONCAT31(local_4._1_3_,8);
              local_a8 = uVar6;
              pvVar2 = FUN_004ecc70(pvVar2,puVar4);
            }
            local_4 = 7;
            if ((uVar6 & 4) != 0) {
              uVar6 = uVar6 & 0xfffffffb;
              FUN_00619730();
            }
            local_4 = CONCAT31(local_4._1_3_,6);
            FUN_004f57b0(param_2,pvVar2);
          }
          FUN_005130d0((int)local_8c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_00504d40(local_8c);
        break;
      case 0x68:
        FUN_00536da0(local_6c,pvVar2,3);
        local_4._0_1_ = 9;
        FUN_004ece30(&local_ac);
        local_4 = CONCAT31(local_4._1_3_,10);
        FUN_00513120((int)local_6c);
        while (local_50 != 0) {
          puVar4 = &local_a4;
          pvVar2 = (void *)FUN_0052bed0((int)local_6c);
          puVar4 = FUN_004025b0(pvVar2,puVar4);
          local_4._0_1_ = 0xb;
          FUN_004f26d0(&local_ac,puVar4);
          local_4._0_1_ = 10;
          FUN_00619730();
          pvVar2 = (void *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0xc;
          if (pvVar2 == (void *)0x0) {
            pvVar2 = (void *)0x0;
          }
          else {
            pvVar2 = FUN_004ecc70(pvVar2,&local_ac);
          }
          local_4 = CONCAT31(local_4._1_3_,10);
          local_94 = 0x30;
          local_90 = 0x3c;
          if ((local_ac >> 0x18 < 0x30) || (0x3b < local_ac >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          pvVar3 = param_2;
          if ((bVar1) && (iVar5 = FUN_0052bed0((int)local_6c), (*(byte *)(iVar5 + 0xac) & 1) != 0))
          {
            pvVar3 = param_3;
          }
          FUN_004f57b0(pvVar3,pvVar2);
          FUN_005130d0((int)local_6c);
        }
        local_4._1_3_ = (uint3)(local_4 >> 8);
        local_4._0_1_ = 9;
        FUN_00619730();
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00536ea0(local_6c);
      }
    }
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  ExceptionList = pvStack_c;
  return;
}

