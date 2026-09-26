
int __thiscall FUN_0051b990(void *this,int param_1)

{
  int iVar1;
  void *pvVar2;
  undefined4 uVar3;
  undefined3 extraout_var;
  bool bVar4;
  undefined4 *puVar5;
  undefined4 uVar6;
  uint local_a8;
  undefined4 local_a4 [7];
  int local_88;
  undefined4 local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined4 local_68 [7];
  int local_4c;
  undefined4 local_48 [7];
  int local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642fb9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004fd450(&local_28,param_1);
  local_4 = 0;
  local_24 = 2;
  local_20 = 2;
  FUN_004fd450(&local_84,param_1);
  local_4._0_1_ = 1;
  local_80 = 1;
  local_7c = 2;
  iVar1 = FUN_004f8340(this,&local_84);
  bVar4 = iVar1 != 0;
  FUN_005266a0(local_a4,this,4);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00513120((int)local_a4);
  while (local_88 != 0) {
    puVar5 = &local_28;
    pvVar2 = (void *)FUN_0052bed0((int)local_a4);
    iVar1 = FUN_0050c8a0(pvVar2,puVar5);
    if ((iVar1 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    uVar6 = 4;
    uVar3 = FUN_0052bed0((int)local_a4);
    FUN_00512f90(local_68,uVar3,uVar6);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00513120((int)local_68);
    while (local_4c != 0) {
      puVar5 = &local_28;
      pvVar2 = (void *)FUN_0052bed0((int)local_68);
      iVar1 = FUN_00529a40(pvVar2,puVar5);
      if ((iVar1 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
      FUN_005130d0((int)local_68);
    }
    puVar5 = &local_84;
    pvVar2 = (void *)FUN_0052bed0((int)local_a4);
    iVar1 = FUN_00566c00(pvVar2,puVar5);
    if ((iVar1 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    uVar6 = 4;
    uVar3 = FUN_0052bed0((int)local_a4);
    FUN_00567340(local_48,uVar3,uVar6);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00513120((int)local_48);
    while (local_2c != 0) {
      puVar5 = &local_84;
      pvVar2 = (void *)FUN_0052bed0((int)local_48);
      iVar1 = FUN_00539ac0(pvVar2,puVar5);
      if ((iVar1 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
      FUN_005130d0((int)local_48);
    }
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4._0_1_ = 3;
    FUN_005673c0(local_48);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00513010(local_68);
    FUN_005130d0((int)local_a4);
  }
  local_a8 = 0x92000121;
  local_4._1_3_ = (uint3)((uint)local_4 >> 8);
  local_4._0_1_ = 5;
  pvVar2 = (void *)FUN_00505f60(&local_a8);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  if ((pvVar2 == (void *)0x0) || (!bVar4)) {
    iVar1 = 0;
  }
  else {
    iVar1 = 1;
  }
  if (pvVar2 != (void *)0x0) {
    bVar4 = FUN_0050d5a0(pvVar2,10,2,param_1);
    if ((CONCAT31(extraout_var,bVar4) == 0) || (iVar1 == 0)) {
      iVar1 = 0;
    }
    else {
      iVar1 = 1;
    }
  }
  local_4._0_1_ = 1;
  FUN_00526720(local_a4);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004fd4d0(&local_84);
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_28);
  ExceptionList = pvStack_c;
  return iVar1;
}

