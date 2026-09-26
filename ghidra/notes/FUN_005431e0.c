
int FUN_005431e0(int *param_1,undefined4 *param_2)

{
  bool bVar1;
  uint unaff_EBP;
  int local_1c [3];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647218;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece80(param_2);
  FUN_004ece30(local_1c);
  bVar1 = false;
  local_4 = 0;
  local_1c[0] = (**(code **)(*param_1 + 0xc))(local_1c);
  if (local_1c[0] != 0) {
    local_1c[1] = 0x90;
    local_1c[2] = 0x98;
    if ((0x8f < unaff_EBP >> 0x18) && (unaff_EBP >> 0x18 < 0x98)) {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_004f26d0(param_1,(undefined4 *)&stack0xffffffe0);
    }
    else {
      local_1c[1] = 0x98;
      local_1c[2] = 0xa0;
      FUN_00619730();
      local_1c[1] = 0x90;
      local_1c[2] = 0x98;
      if (((uint)param_1[0x1d] >> 0x18 < 0x90) || (0x97 < (uint)param_1[0x1d] >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_004f26d0(param_1,param_1 + 0x1d);
      }
      else {
        if (((byte)param_1[9] & 0xc0) == 0x40) {
          puStack_8._0_1_ = 1;
          FUN_004f26d0(param_1,(undefined4 *)&stack0x00000000);
        }
        else {
          puStack_8._0_1_ = 2;
          FUN_004f26d0(param_1,(undefined4 *)&stack0x00000000);
        }
        puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
        FUN_00619730();
      }
    }
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_10;
  return local_1c[0];
}

