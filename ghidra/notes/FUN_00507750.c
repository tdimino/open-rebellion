
undefined4 __thiscall
FUN_00507750(void *this,uint *param_1,undefined4 param_2,undefined4 *param_3,undefined4 *param_4,
            int *param_5,void *param_6)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c;
  undefined4 local_48;
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
  
  puStack_8 = &LAB_00641670;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_3 = 1;
  *param_4 = 1;
  *param_5 = 0;
  local_5c = 1;
  local_58 = 0xffffffff;
  local_4 = 0;
  FUN_00520580(param_6,&local_5c);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_5c = 0x20;
  local_58 = 0x30;
  if ((*param_1 < 0x20) || (bVar1 = true, 0x30 < param_1[1])) {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    local_5c = 0x2c;
    local_58 = 0x2d;
    if ((*param_1 < 0x2c) || (0x2d < param_1[1])) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    *param_4 = 0;
    if (bVar1) {
      FUN_0052d610(local_2c,this,param_2);
      local_4 = 1;
      iVar2 = FUN_00513180((int)local_2c);
      iVar3 = FUN_00509660((int)this);
      local_5c = 0x90;
      *param_5 = iVar3 - iVar2;
      local_58 = 2;
      local_4._0_1_ = 2;
      FUN_00520580(param_6,&local_5c);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_0052d710(local_2c);
      ExceptionList = local_c;
      return 1;
    }
    FUN_00539d70(local_2c,this,param_2);
    local_4 = 3;
    iVar2 = FUN_00513180((int)local_2c);
    iVar3 = FUN_00509650((int)this);
    local_5c = 0x90;
    *param_5 = iVar3 - iVar2;
    local_58 = 1;
    local_4._0_1_ = 4;
    FUN_00520580(param_6,&local_5c);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00539e70(local_2c);
    ExceptionList = local_c;
    return 1;
  }
  local_34 = 0x10;
  local_30 = 0x14;
  local_4 = 5;
  if ((*param_1 < 0x10) || (0x14 < param_1[1])) {
    local_5c = 0x1c;
    local_58 = 0x20;
    if ((*param_1 < 0x1c) || (0x20 < param_1[1])) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      local_54 = 8;
      local_50 = 0x10;
      if ((*param_1 < 8) || (0x10 < param_1[1])) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        local_4c = 0xa0;
        local_48 = 0xb0;
        if ((*param_1 < 0xa0) || (0xb0 < param_1[1])) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) {
          local_44 = 0x30;
          local_40 = 0x40;
          if ((*param_1 < 0x30) || (0x40 < param_1[1])) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) {
            local_3c = 0x40;
            local_38 = 0x80;
            if ((*param_1 < 0x40) || (0x80 < param_1[1])) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (!bVar1) {
              bVar1 = false;
              goto LAB_00507a5d;
            }
          }
        }
      }
    }
  }
  bVar1 = true;
LAB_00507a5d:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    *param_4 = 1;
  }
  else {
    *param_3 = 0;
  }
  ExceptionList = local_c;
  return 1;
}

