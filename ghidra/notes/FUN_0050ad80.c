
int __thiscall FUN_0050ad80(void *this,void *param_1)

{
  ushort uVar1;
  int iVar2;
  void *pvVar3;
  int iVar4;
  uint *puVar5;
  int *piVar6;
  int iVar7;
  uint uVar8;
  int iVar9;
  uint uStack_70;
  uint uStack_6c;
  int local_68;
  int iStack_64;
  void *local_60;
  undefined4 auStack_5c [2];
  undefined4 auStack_54 [2];
  undefined4 local_4c [7];
  int iStack_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641ae0;
  pvStack_c = ExceptionList;
  iVar7 = 0;
  local_68 = 0;
  ExceptionList = &pvStack_c;
  local_60 = this;
  FUN_00539df0(local_2c,this,3,*(uint *)((int)this + 0x24) >> 6 & 3);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  FUN_00513120((int)local_2c);
  iVar2 = local_68;
  while (local_10 != 0) {
    FUN_0052bed0((int)local_2c);
    iVar2 = FUN_0040f340();
    iVar7 = iVar7 + iVar2;
    FUN_005130d0((int)local_2c);
    iVar2 = iVar7;
  }
  local_68 = iVar2;
  if (*(int *)((int)this + 0x5c) < iVar7) {
    pvVar3 = FUN_00539d70(local_4c,this,3);
    local_4._0_1_ = 1;
    iVar4 = FUN_00513180((int)pvVar3);
    local_4._0_1_ = 0;
    FUN_00539e70(local_4c);
    pvVar3 = FUN_00526cf0(local_4c,this,3);
    local_4._0_1_ = 2;
    uStack_70 = FUN_00513180((int)pvVar3);
    local_4._0_1_ = 0;
    FUN_00526df0(local_4c);
    iVar9 = 0;
    iVar2 = FUN_00509650((int)this);
    iVar2 = FUN_0053e130(iVar4 - iVar2,iVar9);
    iStack_64 = iVar2;
    if (iVar2 != 0) {
      FUN_004f4340(auStack_5c);
      local_4._0_1_ = 3;
      FUN_00539d70(local_4c,this,3);
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_00513120((int)local_4c);
      while (iStack_30 != 0) {
        puVar5 = &uStack_6c;
        pvVar3 = (void *)FUN_0052bed0((int)local_4c);
        puVar5 = FUN_004025b0(pvVar3,puVar5);
        local_4._0_1_ = 5;
        FUN_004f44b0(auStack_5c,puVar5,0);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_00619730();
        FUN_005130d0((int)local_4c);
      }
      if ((uStack_70 != 0) && (iVar2 < iVar4)) {
        uStack_6c = 0x20000005;
        local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
        local_4._0_1_ = 6;
        FUN_004f58e0(auStack_5c,&uStack_6c);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_00619730();
      }
      FUN_004f4340(auStack_54);
      local_4 = CONCAT31(local_4._1_3_,7);
      uVar1 = FUN_005f50e0((int)auStack_5c);
      uVar8 = (uint)uVar1;
      for (uStack_6c = 0; (int)uStack_6c < iVar2; uStack_6c = uStack_6c + 1) {
        iVar7 = FUN_0053e290(uVar8 - 1);
        pvVar3 = (void *)thunk_FUN_005f5060((int)auStack_5c);
        if (pvVar3 != (void *)0x0) {
          for (; iVar7 != 0; iVar7 = iVar7 + -1) {
          }
          if (pvVar3 != (void *)0x0) {
            FUN_00403040(pvVar3,&uStack_70);
            local_4._0_1_ = 8;
            FUN_004f58e0(auStack_5c,&uStack_70);
            FUN_004f44b0(auStack_54,&uStack_70,0);
            local_4 = CONCAT31(local_4._1_3_,7);
            FUN_00619730();
            uVar8 = uVar8 - 1;
          }
        }
        iVar2 = iStack_64;
      }
      for (pvVar3 = (void *)thunk_FUN_005f5060((int)auStack_54); pvVar3 != (void *)0x0;
          pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
        puVar5 = (uint *)FUN_00403040(pvVar3,&iStack_64);
        local_4._0_1_ = 9;
        piVar6 = (int *)FUN_00505530(puVar5);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00619730();
        if (piVar6 != (int *)0x0) {
          (**(code **)(*piVar6 + 0xb8))(param_1);
        }
      }
      local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
      local_4._0_1_ = 4;
      FUN_004f4380(auStack_54);
      local_4._0_1_ = 3;
      FUN_00539e70(local_4c);
      local_4._0_1_ = 0;
      FUN_004f4380(auStack_5c);
      iVar7 = local_68;
      this = local_60;
    }
  }
  iVar7 = FUN_0053e140(iVar7,*(int *)((int)this + 0x5c));
  iVar7 = FUN_00509d40(this,iVar7,param_1);
  local_4 = 0xffffffff;
  FUN_00539e70(local_2c);
  ExceptionList = pvStack_c;
  return iVar7;
}

