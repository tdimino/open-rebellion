
int __thiscall FUN_00592c80(void *this,int param_1)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  uint uVar4;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  int iVar5;
  int *local_48;
  int *local_44;
  int local_40;
  int iStack_3c;
  int local_38;
  void *local_34;
  uint local_30;
  uint local_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006508f0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_34 = this;
  iVar3 = FUN_00520af0((int)this);
  if (iVar3 == 0) {
    ExceptionList = local_c;
    return 1;
  }
  local_44 = (int *)0x0;
  bVar1 = FUN_00521070(this,(int *)&local_44);
  bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  if (local_44 != (int *)0x0) {
    local_30 = 0x90;
    local_2c = 0x98;
    local_4 = 0;
    uVar4 = (**(code **)(*local_44 + 4))();
    if ((uVar4 < local_30) || (local_2c <= uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar2) {
      bVar2 = FUN_0050d5a0(local_44,6,*(uint *)((int)this + 0x24) >> 6 & 3,param_1);
      if ((CONCAT31(extraout_var_00,bVar2) == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
    }
  }
  local_48 = (int *)0x0;
  local_40 = 0;
  FUN_004ece30(&local_38);
  local_4 = 1;
  FUN_004ece30(&iStack_3c);
  local_4._0_1_ = 2;
  bVar2 = FUN_00521160(this,&local_48);
  if ((CONCAT31(extraout_var_01,bVar2) == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  bVar2 = FUN_00521030(this,&local_40);
  if ((CONCAT31(extraout_var_02,bVar2) == 0) || (!bVar1)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if ((local_48 == (int *)0x0) || (local_40 == 0)) goto LAB_00592f17;
  local_30 = 0x90;
  local_2c = 0x98;
  local_4._0_1_ = 3;
  uVar4 = (**(code **)(*local_48 + 4))();
  if ((uVar4 < local_30) || (local_2c <= uVar4)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4._0_1_ = 2;
  FUN_00619730();
  if (bVar1) goto LAB_00592f17;
  local_30 = 0x98;
  local_2c = 0xa0;
  local_4._0_1_ = 4;
  uVar4 = (**(code **)(*local_48 + 4))();
  if ((uVar4 < local_30) || (local_2c <= uVar4)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4._0_1_ = 2;
  FUN_00619730();
  if (bVar1) goto LAB_00592f17;
  FUN_004fd450(&uStack_28,param_1);
  uStack_24 = 1;
  uStack_20 = 3;
  local_4 = CONCAT31(local_4._1_3_,5);
  iVar5 = (**(code **)(*local_48 + 0xc))(&local_38);
  if ((iVar5 == 0) || (iVar3 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = (**(code **)(*local_44 + 0xc))(&local_40);
  if ((iVar3 == 0) || (!bVar1)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if (*(int *)((int)local_34 + 0x78) == iStack_3c) {
    iVar5 = (**(code **)(*local_48 + 0x24))(&uStack_28);
    if ((iVar5 == 0) || (iVar3 == 0)) {
LAB_00592f07:
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
  }
  else if (*(int *)((int)local_34 + 0x78) == local_38) {
    iVar5 = FUN_004f7f20(local_48,&uStack_28);
    if ((iVar5 == 0) || (iVar3 == 0)) goto LAB_00592f07;
    iVar3 = 1;
  }
  local_4._0_1_ = 2;
  FUN_004fd4d0(&uStack_28);
LAB_00592f17:
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar3;
}

