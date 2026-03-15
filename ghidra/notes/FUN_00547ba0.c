
int __thiscall
FUN_00547ba0(void *this,int param_1,int param_2,int param_3,uint *param_4,uint *param_5,int *param_6
            ,uint *param_7,int param_8,undefined4 param_9,void *param_10,int param_11,int param_12,
            uint param_13)

{
  bool bVar1;
  short sVar2;
  int iVar3;
  int *this_00;
  int *this_01;
  uint *puVar4;
  uint uVar5;
  int iVar6;
  undefined3 extraout_var;
  bool bVar7;
  uint local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28 [5];
  undefined4 local_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006479e8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_3c);
  bVar7 = false;
  local_4 = 0;
  FUN_004ece30(&local_40);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_44);
  local_4._0_1_ = 2;
  FUN_004fd450(local_28,param_13);
  local_4 = CONCAT31(local_4._1_3_,3);
  local_14 = FUN_004fd2f0();
  iVar3 = thunk_FUN_00506e60();
  if ((iVar3 != 0) && ((param_3 == 1 || (param_3 == 2)))) {
    bVar7 = true;
  }
  this_00 = FUN_004f7d50(this,param_5,param_3,local_28,1,1);
  if ((this_00 == (int *)0x0) || (!bVar7)) {
    bVar7 = false;
  }
  else {
    bVar7 = true;
  }
  if (bVar7) {
    sVar2 = FUN_005f50e0(param_8);
    bVar7 = sVar2 != 0;
  }
  if (param_3 == 1) {
    iVar3 = 1;
  }
  else if (param_3 == 2) {
    iVar3 = 2;
  }
  else {
    iVar3 = 0;
  }
  this_01 = FUN_004f2d10(iVar3,param_4);
  if ((this_01 == (int *)0x0) || (!bVar7)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if (this_01 != (int *)0x0) {
    puVar4 = FUN_004025b0(this_01,&param_13);
    local_4._0_1_ = 4;
    FUN_004f26d0(&local_40,puVar4);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00619730();
  }
  if (iVar3 != 0) {
    local_30 = 0x90;
    local_2c = 0x98;
    uVar5 = *param_7 >> 0x18;
    local_4 = CONCAT31(local_4._1_3_,5);
    if ((uVar5 < 0x90) || (0x97 < uVar5)) {
      local_38 = 0x98;
      local_34 = 0xa0;
      if ((uVar5 < 0x98) || (0x9f < uVar5)) {
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
      FUN_00619730();
      if (bVar7) goto LAB_00547d2c;
      iVar3 = 0;
    }
    else {
LAB_00547d2c:
      iVar3 = 1;
    }
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00619730();
  }
  if (this_01 != (int *)0x0) {
    iVar6 = (**(code **)(*this_01 + 0xc))(&local_44);
    if ((iVar6 == 0) || (iVar3 == 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
  }
  if (iVar3 == 0) goto LAB_00547dcd;
  local_38 = 0x90;
  local_34 = 0x98;
  local_44 = local_44 >> 0x18;
  local_4 = CONCAT31(local_4._1_3_,6);
  if ((local_44 < 0x90) || (0x97 < local_44)) {
    local_30 = 0x98;
    local_2c = 0xa0;
    if ((local_44 < 0x98) || (0x9f < local_44)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    FUN_00619730();
    if (bVar7) goto LAB_00547dba;
    iVar3 = 0;
  }
  else {
LAB_00547dba:
    iVar3 = 1;
  }
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_00619730();
LAB_00547dcd:
  if (this_00 != (int *)0x0) {
    puVar4 = FUN_004ece40(&local_44);
    if (puVar4 != (uint *)0x0) {
      FUN_00521980(this_00,1,local_28);
      FUN_00521780(this_00,param_1,local_28);
      param_13 = FUN_00521800(this_00,param_2,local_28);
      FUN_00521a00(this_00,(int *)param_7,local_28);
      FUN_00521ab0(this_00,(int *)param_4,local_28);
      iVar3 = FUN_00521b60(this_00,param_6,local_28);
      iVar6 = FUN_00521c10(this_00,(int *)&local_44,local_28);
      if ((iVar6 == 0) || (iVar3 == 0)) {
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
      iVar3 = FUN_005221a0(this_00,param_11,local_28);
      if ((iVar3 == 0) || (!bVar7)) {
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
      iVar3 = FUN_00522210(this_00,param_12,local_28);
      if ((iVar3 == 0) || (!bVar7)) {
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
      if (param_10 != (void *)0x0) {
        bVar1 = FUN_00522a60(this_00,param_10);
        if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar7)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
      }
      iVar3 = (**(code **)(*this_00 + 0x1e0))(param_8,param_9,local_28);
      if ((iVar3 == 0) || (!bVar7)) {
        iVar3 = 0;
      }
      else {
        iVar3 = 1;
      }
    }
  }
  local_4._0_1_ = 2;
  FUN_004fd4d0(local_28);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar3;
}

