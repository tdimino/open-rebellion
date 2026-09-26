
void __thiscall FUN_0045ce80(void *this,void *param_1)

{
  uint uVar1;
  bool bVar2;
  uint *puVar3;
  void *pvVar4;
  void *pvVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  void **ppvVar9;
  void *local_9c;
  void *local_98;
  void *local_94;
  uint uStack_90;
  undefined4 auStack_8c [7];
  int iStack_70;
  undefined4 local_6c [8];
  undefined4 local_4c [8];
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630bf9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_94 = this;
  puVar3 = FUN_004025b0(param_1,(uint *)&local_9c);
  local_4 = 0;
  pvVar4 = (void *)FUN_0060a860((void *)((int)this + 0x174),(ushort)*puVar3 | 0x80000);
  local_4 = 0xffffffff;
  local_98 = pvVar4;
  FUN_00619730();
  if (pvVar4 != (void *)0x0) {
    uVar1 = *(uint *)((int)param_1 + 0x24);
    pvVar4 = FUN_00526fd0(local_2c,param_1,3);
    local_4 = 1;
    pvVar5 = FUN_00504c40(local_4c,param_1,3);
    local_4._0_1_ = 2;
    local_9c = FUN_005039d0(local_6c,param_1,3);
    local_4._0_1_ = 3;
    iVar6 = FUN_00513180((int)pvVar4);
    iVar7 = FUN_00513180((int)pvVar5);
    iVar8 = FUN_00513180((int)local_9c);
    iVar8 = iVar6 + iVar7 + iVar8;
    local_4._0_1_ = 2;
    FUN_00503ad0(local_6c);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00504d40(local_4c);
    local_4 = 0xffffffff;
    FUN_005270d0(local_2c);
    FUN_00536da0(auStack_8c,param_1,3);
    local_4 = 4;
    FUN_00513120((int)auStack_8c);
    while (iStack_70 != 0) {
      ppvVar9 = &local_9c;
      pvVar4 = (void *)FUN_0052bed0((int)auStack_8c);
      puVar3 = FUN_0042d170(pvVar4,ppvVar9);
      local_4 = CONCAT31(local_4._1_3_,5);
      iVar6 = FUN_004ece60(puVar3);
      if (iVar6 == 0) {
LAB_0045d01b:
        bVar2 = true;
      }
      else {
        iVar6 = FUN_0052bed0((int)auStack_8c);
        uStack_90 = *(uint *)(iVar6 + 0x78) >> 8 & 1;
        if (uStack_90 != 0) goto LAB_0045d01b;
        bVar2 = false;
      }
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_00619730();
      if (bVar2) {
        iVar8 = iVar8 + 1;
      }
      FUN_005130d0((int)auStack_8c);
    }
    FUN_0045d140(local_94,local_98,uVar1 >> 6 & 3,iVar8);
    local_4 = 0xffffffff;
    FUN_00536ea0(auStack_8c);
  }
  ExceptionList = local_c;
  return;
}

