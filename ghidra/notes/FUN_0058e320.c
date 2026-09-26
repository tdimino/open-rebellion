
undefined4 __thiscall FUN_0058e320(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar4;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  void *pvVar5;
  undefined4 uVar6;
  bool bVar7;
  void *pvVar8;
  int local_6c [2];
  uint local_64 [7];
  uint local_48 [7];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650130;
  pvStack_c = ExceptionList;
  pvVar8 = *(void **)((int)this + 0x10);
  iVar4 = *(int *)((int)this + 0x14);
  ExceptionList = &pvStack_c;
  FUN_0058d680(local_6c,(void *)((int)this + 8));
  local_4 = 0;
  iVar2 = FUN_0058ca70((void *)((int)this + 8),local_6c,param_1);
  bVar7 = iVar2 != 0;
  if (*(int *)((int)this + 0x28) != 0) {
    iVar2 = iVar4;
    pvVar5 = param_1;
    iVar3 = FUN_0055a0e0();
    bVar1 = FUN_0050c9f0(pvVar8,iVar3,iVar2,pvVar5);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar7)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
  }
  if (*(int *)((int)this + 0x2c) == 0) {
    if (*(int *)((int)this + 0x28) == 0) goto LAB_0058e4d1;
    FUN_00562ae0(local_64);
    local_4._0_1_ = 3;
    FUN_0055d900(iVar4,1,local_64);
    FUN_00518ac0(*(void **)((int)this + 0x10),local_64,param_1);
    FUN_00562ae0(local_48);
    local_4._0_1_ = 4;
    bVar7 = FUN_0055d900(iVar4,0,local_48);
    bVar1 = FUN_00518ac0(*(void **)((int)this + 0x10),local_48,param_1);
    if ((CONCAT31(extraout_var_02,bVar1) == 0) || (CONCAT31(extraout_var_01,bVar7) == 0)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    local_4._0_1_ = 3;
    FUN_00562b30();
  }
  else {
    FUN_00562ae0(local_48);
    local_4._0_1_ = 1;
    FUN_0055da90(iVar4,1,local_48);
    FUN_00518a70(local_48,param_1);
    FUN_00562ae0(local_64);
    local_4._0_1_ = 2;
    bVar7 = FUN_0055da90(iVar4,0,local_64);
    iVar4 = FUN_00518a70(local_64,param_1);
    if ((iVar4 == 0) || (CONCAT31(extraout_var_00,bVar7) == 0)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    local_4._0_1_ = 1;
    FUN_00562b30();
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00562b30();
LAB_0058e4d1:
  iVar4 = FUN_0050a660(*(void **)((int)this + 0x10),0,param_1);
  if ((iVar4 == 0) || (!bVar7)) {
    uVar6 = 0;
  }
  else {
    uVar6 = 1;
  }
  FUN_004ffef0(local_2c,*(undefined4 *)((int)this + 0x10),4,*(undefined4 *)((int)this + 0x14));
  local_4 = CONCAT31(local_4._1_3_,5);
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    iVar4 = 0;
    pvVar8 = param_1;
    pvVar5 = (void *)FUN_0052bed0((int)local_2c);
    FUN_004fe460(pvVar5,iVar4,pvVar8);
    FUN_005130d0((int)local_2c);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_004fff70(local_2c);
  local_4 = 0xffffffff;
  FUN_0058d6c0(local_6c);
  ExceptionList = pvStack_c;
  return uVar6;
}

