
int __thiscall FUN_004ffa70(void *this,int param_1,void *param_2)

{
  int iVar1;
  int iVar2;
  uint local_54 [17];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006406c0;
  pvStack_c = ExceptionList;
  local_54[2] = 1;
  local_54[3] = -1;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,local_54 + 2);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = FUN_004f9860(this,param_1,param_2);
  if (iVar1 != 0) {
    if (*(int *)((int)param_2 + 4) == -1) {
      FUN_00520570(local_54 + 2);
      local_54[0] = 8;
      local_54[1] = 1;
      local_4._0_1_ = 2;
      local_4._1_3_ = 0;
      FUN_00520580(local_54 + 2,local_54);
      local_4._0_1_ = 1;
      FUN_00619730();
      FUN_00502db0(local_54 + 10,this,3);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00513120((int)(local_54 + 10));
      while (local_10 != 0) {
        if (local_54[3] == -1) goto LAB_004ffc1d;
        iVar2 = FUN_0052bed0((int)(local_54 + 10));
        local_54[0] = *(uint *)(iVar2 + 0x50) >> 2 & 1;
        if (local_54[0] == 0) {
          local_54[4] = 1;
          local_54[5] = 0x10;
          local_4._0_1_ = 4;
          FUN_00520580(local_54 + 2,local_54 + 4);
          local_4 = CONCAT31(local_4._1_3_,3);
        }
        else {
          iVar2 = FUN_0052bed0((int)(local_54 + 10));
          local_54[0] = *(uint *)(iVar2 + 0x50) >> 4 & 1;
          if (local_54[0] == 0) {
            local_54[8] = 1;
            local_54[9] = 0xffffffff;
            local_4._0_1_ = 6;
            FUN_00520580(local_54 + 2,local_54 + 8);
            local_4 = CONCAT31(local_4._1_3_,3);
          }
          else {
            local_54[6] = 1;
            local_54[7] = 0x11;
            local_4._0_1_ = 5;
            FUN_00520580(local_54 + 2,local_54 + 6);
            local_4 = CONCAT31(local_4._1_3_,3);
          }
        }
        FUN_00619730();
        FUN_005130d0((int)(local_54 + 10));
      }
      if (local_54[3] != -1) {
        FUN_00520580(param_2,local_54 + 2);
      }
LAB_004ffc1d:
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00502eb0(local_54 + 10);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    if (iVar1 != 0) {
      ExceptionList = pvStack_c;
      return iVar1;
    }
  }
  local_54[8] = 1;
  local_54[9] = 1;
  local_4 = 7;
  FUN_00520580(param_2,local_54 + 8);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar1;
}

