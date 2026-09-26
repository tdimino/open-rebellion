
void __thiscall FUN_00496f50(void *this,uint param_1,uint *param_2,void *param_3,undefined4 param_4)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  uint *puVar4;
  void *pvVar5;
  void *pvVar6;
  int iVar7;
  uint local_58;
  uint local_54;
  uint local_50;
  uint local_4c;
  uint local_48;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006364bc;
  local_c = ExceptionList;
  bVar1 = false;
  bVar2 = false;
  if (param_1 == 0x14f) {
    ExceptionList = &local_c;
    piVar3 = FUN_004f3220(*(int *)((int)this + 0x20),param_2);
    if (piVar3 == (int *)0x0) {
      ExceptionList = local_c;
      return;
    }
    puVar4 = FUN_004025b0(piVar3,&local_50);
    local_4 = 6;
    FUN_004f26d0((void *)((int)this + 0x5c),puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (param_3 == (void *)0x0) {
      *(undefined4 *)((int)this + 0x40) = 1;
      FUN_004fcd00(local_2c,piVar3,1);
      local_4 = 7;
      FUN_00513120((int)local_2c);
      pvVar5 = (void *)FUN_0052bed0((int)local_2c);
      while (pvVar5 != (void *)0x0) {
        pvVar6 = (void *)FUN_00618b70(0x1c);
        local_4._0_1_ = 8;
        param_3 = pvVar6;
        if (pvVar6 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          puVar4 = FUN_004025b0(pvVar5,&local_4c);
          bVar2 = true;
          local_4 = CONCAT31(local_4._1_3_,9);
          pvVar6 = FUN_004ecc70(pvVar6,puVar4);
        }
        local_4 = 8;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 7;
        puVar4 = FUN_004025b0(pvVar5,&local_48);
        local_34 = 0x30;
        local_30 = 0x3c;
        local_4._0_1_ = 10;
        if ((*puVar4 >> 0x18 < 0x30) || (0x3b < *puVar4 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00619730();
        if (bVar1) {
          if ((*(byte *)((int)pvVar5 + 0xac) & 1) == 0) {
            pvVar5 = (void *)((int)this + 0x70);
          }
          else {
            pvVar5 = (void *)((int)this + 0x98);
          }
        }
        else {
          pvVar5 = (void *)((int)this + 0x70);
        }
        FUN_004f57b0(pvVar5,pvVar6);
        FUN_005130d0((int)local_2c);
        pvVar5 = (void *)FUN_0052bed0((int)local_2c);
      }
      local_4 = 0xffffffff;
      FUN_004fcd80(local_2c);
      ExceptionList = local_c;
      return;
    }
  }
  else {
    if (param_1 != 0x150) {
      return;
    }
    ExceptionList = &local_c;
    piVar3 = FUN_004f3220(*(int *)((int)this + 0x20),param_2);
    *(undefined4 *)((int)this + 100) = 1;
    if (piVar3 == (int *)0x0) {
      ExceptionList = local_c;
      return;
    }
    puVar4 = FUN_004025b0(piVar3,(uint *)&param_2);
    local_4 = 0;
    FUN_004f26d0((void *)((int)this + 0x5c),puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (param_3 == (void *)0x0) {
      *(undefined4 *)((int)this + 0x40) = 1;
      FUN_004fcd00(local_2c,piVar3,1);
      local_4._0_1_ = 1;
      local_4._1_3_ = 0;
      FUN_00513120((int)local_2c);
      pvVar5 = (void *)FUN_0052bed0((int)local_2c);
      while (pvVar5 != (void *)0x0) {
        pvVar6 = (void *)FUN_00618b70(0x1c);
        local_4._0_1_ = 2;
        param_3 = pvVar6;
        if (pvVar6 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          puVar4 = FUN_004025b0(pvVar5,&param_1);
          bVar1 = true;
          local_4 = CONCAT31(local_4._1_3_,3);
          pvVar6 = FUN_004ecc70(pvVar6,puVar4);
        }
        local_4 = 2;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 1;
        puVar4 = FUN_004025b0(pvVar5,&local_58);
        local_44 = 0x10;
        local_40 = 0x14;
        local_4._0_1_ = 4;
        if ((*puVar4 >> 0x18 < 0x10) || (0x13 < *puVar4 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 1;
        FUN_00619730();
        if (bVar2) {
          if ((*(uint *)((int)pvVar5 + 0x24) >> 6 & 3) == *(uint *)((int)this + 0x68)) {
            pvVar5 = (void *)((int)this + 0x70);
          }
          else {
            pvVar5 = (void *)((int)this + 0x78);
          }
        }
        else {
          puVar4 = FUN_004025b0(pvVar5,&local_54);
          local_3c = 0x30;
          local_38 = 0x3c;
          local_4._0_1_ = 5;
          if ((*puVar4 >> 0x18 < 0x30) || (0x3b < *puVar4 >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 1;
          FUN_00619730();
          if (bVar2) {
            if ((*(byte *)((int)pvVar5 + 0xac) & 1) == 0) {
              pvVar5 = (void *)((int)this + 0x70);
            }
            else {
              iVar7 = FUN_004edc80((int)pvVar5);
              if (iVar7 == *(int *)((int)this + 0x68)) {
                pvVar5 = (void *)((int)this + 0x90);
              }
              else {
                pvVar5 = (void *)((int)this + 0x98);
              }
            }
          }
          else {
            pvVar5 = (void *)((int)this + 0x70);
          }
        }
        FUN_004f57b0(pvVar5,pvVar6);
        FUN_005130d0((int)local_2c);
        pvVar5 = (void *)FUN_0052bed0((int)local_2c);
      }
      local_4 = 0xffffffff;
      FUN_004fcd80(local_2c);
      ExceptionList = local_c;
      return;
    }
  }
  *(undefined4 *)((int)this + 0x68) = param_4;
  ExceptionList = local_c;
  return;
}

