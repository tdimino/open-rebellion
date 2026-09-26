
bool __thiscall FUN_00522fc0(void *this,uint *param_1,int param_2,void *param_3)

{
  void *pvVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  void *this_00;
  int iVar5;
  int *piVar6;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  int local_38 [2];
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643aa0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar2 = FUN_0053a000((int)this);
  local_38[0] = 0;
  if (CONCAT31(extraout_var,bVar2) == 0) {
    bVar2 = false;
  }
  else {
    local_30 = 0x30;
    local_2c = 0x40;
    if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
      bVar2 = false;
      FUN_00619730();
    }
    else {
      bVar2 = true;
      FUN_00619730();
    }
  }
  if (!bVar2) {
    ExceptionList = local_c;
    return false;
  }
  bVar3 = FUN_00520bd0(this,param_1);
  bVar4 = FUN_00520bf0(this,param_1);
  bVar2 = FUN_00520c10(this,param_1);
  local_38[0] = CONCAT31(extraout_var_02,bVar2);
  this_00 = (void *)FUN_00505d40(param_1);
  bVar2 = this_00 != (void *)0x0;
  if (!bVar2) {
    ExceptionList = local_c;
    return false;
  }
  if (((param_2 == 0) && ((*(uint *)((int)this_00 + 0x50) & 0x20) != 0)) &&
     ((*(uint *)((int)this_00 + 0x50) & 0x800) == 0)) {
    iVar5 = FUN_004f7640(this_00,0,param_3);
    bVar2 = iVar5 != 0;
  }
  iVar5 = FUN_00534560(this_00,param_2,param_3);
  pvVar1 = param_3;
  if ((iVar5 == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_004fd450(local_28,(int)param_3);
  local_4 = 0;
  FUN_004fd620(local_28,1,1);
  FUN_004fd620(local_28,2,1);
  piVar6 = (int *)FUN_004ece30(&param_3);
  local_4._0_1_ = 1;
  iVar5 = FUN_00534230(this_00,piVar6,local_28);
  if ((iVar5 == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  local_4._0_1_ = 0;
  FUN_00619730();
  piVar6 = (int *)FUN_0042dbe0(&param_3);
  local_4._0_1_ = 2;
  iVar5 = FUN_005342e0(this_00,piVar6,pvVar1);
  if ((iVar5 == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  local_4._0_1_ = 0;
  FUN_00619730();
  if (CONCAT31(extraout_var_00,bVar3) == 0) {
    if (CONCAT31(extraout_var_01,bVar4) == 0) {
      if (local_38[0] == 0) goto LAB_00523285;
      FUN_004f4390(local_38,(int)this + 0x94);
      local_4._0_1_ = 5;
      FUN_004f58e0(local_38,param_1);
      iVar5 = FUN_00521fb0(this,local_38,pvVar1);
      if ((iVar5 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
    else {
      FUN_004f4390(local_38,(int)this + 0x8c);
      local_4._0_1_ = 4;
      FUN_004f58e0(local_38,param_1);
      iVar5 = FUN_00521ef0(this,local_38,pvVar1);
      if ((iVar5 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
  }
  else {
    FUN_004f4390(local_38,(int)this + 0x84);
    local_4._0_1_ = 3;
    FUN_004f58e0(local_38,param_1);
    iVar5 = FUN_00521e30(this,local_38,pvVar1);
    if ((iVar5 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
  }
  local_4._0_1_ = 0;
  FUN_004f4380(local_38);
LAB_00523285:
  iVar5 = FUN_00522480(this,pvVar1);
  if ((iVar5 == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if (*(uint *)((int)this + 0x7c) == *param_1) {
    bVar3 = FUN_00522870(this,pvVar1);
    if ((CONCAT31(extraout_var_03,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
  }
  bVar3 = FUN_005229c0(this,pvVar1);
  if ((CONCAT31(extraout_var_04,bVar3) == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return bVar2;
}

