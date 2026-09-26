
undefined4 __thiscall
FUN_00500b40(void *this,uint *param_1,undefined4 param_2,undefined4 *param_3,undefined4 *param_4,
            int *param_5,void *param_6)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00640918;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_3 = 1;
  local_34 = 1;
  *param_4 = 0;
  *param_5 = 0;
  local_30 = 0x17;
  local_4 = 0;
  FUN_00520580(param_6,&local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_34 = 0x1c;
  local_30 = 0x20;
  if ((*param_1 < 0x1c) || (bVar1 = true, 0x20 < param_1[1])) {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    *param_4 = 0;
    FUN_005039d0(local_2c,this,param_2);
    local_4 = 1;
    iVar2 = FUN_00513180((int)local_2c);
    iVar3 = (**(code **)(*(int *)this + 0x26c))();
    local_34 = 0x14;
    local_30 = 1;
    *param_5 = iVar3 - iVar2;
    local_4._0_1_ = 2;
    FUN_00520580(param_6,&local_34);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00503ad0(local_2c);
  }
  else {
    local_34 = 0x10;
    local_30 = 0x14;
    if ((*param_1 < 0x10) || (bVar1 = true, 0x14 < param_1[1])) {
      bVar1 = false;
    }
    FUN_00619730();
    if (bVar1) {
      *param_4 = 0;
      FUN_00504c40(local_2c,this,param_2);
      local_4 = 3;
      iVar2 = FUN_00513180((int)local_2c);
      iVar3 = (**(code **)(*(int *)this + 0x270))();
      local_34 = 0x14;
      *param_5 = iVar3 - iVar2;
      local_30 = 2;
      local_4._0_1_ = 4;
      FUN_00520580(param_6,&local_34);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00504d40(local_2c);
    }
    else {
      local_34 = 0x30;
      local_30 = 0x40;
      if ((*param_1 < 0x30) || (0x40 < param_1[1])) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        *param_4 = 1;
      }
      else {
        *param_3 = 0;
      }
    }
  }
  ExceptionList = local_c;
  return 1;
}

