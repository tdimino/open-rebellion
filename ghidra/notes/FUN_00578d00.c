
uint __thiscall FUN_00578d00(void *this,int *param_1)

{
  int *this_00;
  bool bVar1;
  uint *puVar2;
  uint *puVar3;
  undefined3 extraout_var;
  uint uVar4;
  int *local_24;
  uint local_20 [5];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_0064d178;
  pvStack_c = ExceptionList;
  uVar4 = 1;
  local_24 = (int *)0x0;
  local_20[1] = 1;
  local_20[2] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,local_20 + 1);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_20[0] = 0;
  if (this_00[1] == -1) {
    local_20[0] = thunk_FUN_00506e60();
    uVar4 = (uint)(local_20[0] != 0);
  }
  if ((uVar4 != 0) && (this_00[1] == -1)) {
    local_20[1] = 1;
    local_20[2] = 0x12;
    local_4 = 1;
    uVar4 = FUN_00504e60((uint *)((int)this + 0x3c),&local_24,local_20 + 1,this_00);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  param_1 = (int *)0x0;
  local_20[3] = 1;
  local_20[4] = 0xffffffff;
  local_4 = 2;
  if (uVar4 != 0) {
    if (this_00[1] == -1) {
      local_20[1] = 1;
      local_20[2] = 0x22;
      local_4._0_1_ = 3;
      local_4._1_3_ = 0;
      uVar4 = FUN_00504e60((uint *)((int)this + 0x48),&param_1,local_20 + 1,local_20 + 3);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
      if (uVar4 == 0) goto LAB_00578f00;
      if (param_1 == (int *)0x0) {
        FUN_00520580(this_00,local_20 + 3);
      }
    }
    if (uVar4 != 0) {
      if (this_00[1] == -1) {
        uVar4 = (**(code **)(*local_24 + 0x70))((int)this + 0x40,this_00);
      }
      if (uVar4 != 0) {
        if (this_00[1] == -1) {
          puVar2 = FUN_004025b0(param_1,local_20 + 1);
          local_4._0_1_ = 4;
          uVar4 = FUN_005542f0(local_24,puVar2,(int *)((int)this + 0x40),this_00);
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_00619730();
        }
        if ((uVar4 != 0) && (this_00[1] == -1)) {
          puVar2 = FUN_004025b0(param_1,local_20);
          local_4._0_1_ = 5;
          puVar3 = FUN_004025b0(local_24,local_20 + 1);
          local_4._0_1_ = 6;
          bVar1 = FUN_00515390(puVar3,puVar2,this_00);
          uVar4 = CONCAT31(extraout_var,bVar1);
          local_4._0_1_ = 5;
          FUN_00619730();
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_00619730();
        }
      }
    }
  }
LAB_00578f00:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar4;
}

