
bool FUN_0054a910(uint param_1,uint param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  uint auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  puStack_8 = &LAB_00647da8;
  pvStack_c = ExceptionList;
  bVar1 = true;
  if (param_1 != 0) {
    auStack_14[1] = 0x32000242;
    auStack_14[0] = 0x64000044;
    param_1 = 0x33000243;
    iStack_4._0_1_ = 2;
    iStack_4._1_3_ = 0;
    ExceptionList = &pvStack_c;
    bVar1 = FUN_005479c0(&param_1,auStack_14,auStack_14 + 1,param_2);
    bVar1 = CONCAT31(extraout_var,bVar1) != 0;
    iStack_4._0_1_ = 1;
    FUN_00619730();
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_00619730();
    iStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return bVar1;
}

