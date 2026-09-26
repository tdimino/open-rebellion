
int __thiscall FUN_0055f7b0(void *this,void *param_1)

{
  void *pvVar1;
  bool bVar2;
  bool bVar3;
  void *pvVar4;
  void *pvVar5;
  int iVar6;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  undefined3 extraout_var_05;
  undefined3 extraout_var_06;
  undefined3 extraout_var_07;
  undefined3 extraout_var_08;
  undefined3 extraout_var_09;
  undefined3 extraout_var_10;
  undefined3 extraout_var_11;
  undefined3 extraout_var_12;
  undefined3 extraout_var_13;
  undefined3 extraout_var_14;
  int local_1c;
  void *local_18;
  undefined4 local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a0a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_14);
  local_18 = (void *)0x90000109;
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  pvVar4 = (void *)FUN_00505f60((uint *)&local_18);
  local_4._0_1_ = 0;
  FUN_00619730();
  local_18 = (void *)0x92000121;
  local_4._0_1_ = 2;
  pvVar5 = (void *)FUN_00505f60((uint *)&local_18);
  local_4._0_1_ = 0;
  FUN_00619730();
  iVar6 = FUN_00506110();
  pvVar1 = param_1;
  local_18 = (void *)0x0;
  if (iVar6 != 0) {
    local_18 = *(void **)(iVar6 + 0x1c);
  }
  if (((pvVar4 == (void *)0x0) || (pvVar5 == (void *)0x0)) || (local_18 == (void *)0x0)) {
    iVar6 = 0;
  }
  else {
    local_10 = 0x34000280;
    local_4._0_1_ = 3;
    bVar2 = FUN_0055fe70(&local_10,pvVar4,param_1);
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x35000281;
    local_4._0_1_ = 4;
    bVar3 = FUN_00562680((uint *)&param_1,pvVar1);
    if ((CONCAT31(extraout_var_00,bVar3) == 0) || (CONCAT31(extraout_var,bVar2) == 0)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x38000380;
    local_4._0_1_ = 5;
    bVar3 = FUN_00562680((uint *)&param_1,pvVar1);
    if ((CONCAT31(extraout_var_01,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x38000381;
    local_4._0_1_ = 6;
    bVar3 = FUN_00562680((uint *)&param_1,pvVar1);
    if ((CONCAT31(extraout_var_02,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x38000382;
    local_4._0_1_ = 7;
    bVar3 = FUN_00562680((uint *)&param_1,pvVar1);
    if ((CONCAT31(extraout_var_03,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x38000383;
    local_4._0_1_ = 8;
    bVar3 = FUN_00562680((uint *)&param_1,pvVar1);
    if ((CONCAT31(extraout_var_04,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x3800038c;
    local_4._0_1_ = 9;
    bVar3 = FUN_00562680((uint *)&param_1,pvVar1);
    if ((CONCAT31(extraout_var_05,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    iVar6 = FUN_0055d7d0(2,*(int *)((int)this + 0x5c),&local_1c);
    if ((iVar6 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    param_1 = (void *)0x0;
    if (0 < local_1c) {
      do {
        bVar3 = FUN_00562610(2,&local_14,pvVar1);
        if ((CONCAT31(extraout_var_06,bVar3) == 0) || (!bVar2)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        param_1 = (void *)((int)param_1 + 1);
      } while ((int)param_1 < local_1c);
    }
    param_1 = (void *)0x30000240;
    local_4._0_1_ = 10;
    bVar3 = FUN_0055fe70((uint *)&param_1,local_18,pvVar1);
    if ((CONCAT31(extraout_var_07,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x32000242;
    local_4._0_1_ = 0xb;
    bVar3 = FUN_0055fe70((uint *)&param_1,pvVar5,pvVar1);
    if ((CONCAT31(extraout_var_08,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x31000241;
    local_4._0_1_ = 0xc;
    bVar3 = FUN_0055fe70((uint *)&param_1,pvVar5,pvVar1);
    if ((CONCAT31(extraout_var_09,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x33000243;
    local_4._0_1_ = 0xd;
    bVar3 = FUN_0055fe70((uint *)&param_1,pvVar5,pvVar1);
    if ((CONCAT31(extraout_var_10,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x38000343;
    local_4._0_1_ = 0xe;
    bVar3 = FUN_0055fe70((uint *)&param_1,pvVar5,pvVar1);
    if ((CONCAT31(extraout_var_11,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    param_1 = (void *)0x38000344;
    local_4._0_1_ = 0xf;
    bVar3 = FUN_0055fe70((uint *)&param_1,pvVar5,pvVar1);
    if ((CONCAT31(extraout_var_12,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    local_10 = 0x38000341;
    local_4._0_1_ = 0x10;
    bVar3 = FUN_0055fe70(&local_10,pvVar5,pvVar1);
    if ((CONCAT31(extraout_var_13,bVar3) == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    iVar6 = FUN_0055d7d0(1,*(int *)((int)this + 0x5c),&local_1c);
    if ((iVar6 == 0) || (!bVar2)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    param_1 = (void *)0x0;
    if (0 < local_1c) {
      do {
        bVar2 = FUN_00562610(1,&local_14,pvVar1);
        if ((CONCAT31(extraout_var_14,bVar2) == 0) || (iVar6 == 0)) {
          iVar6 = 0;
        }
        else {
          iVar6 = 1;
        }
        param_1 = (void *)((int)param_1 + 1);
      } while ((int)param_1 < local_1c);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar6;
}

