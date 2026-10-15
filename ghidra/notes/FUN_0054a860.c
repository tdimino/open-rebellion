
bool FUN_0054a860(int param_1,undefined4 param_2)

{
  int iVar1;
  bool bVar2;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  puStack_8 = &LAB_00647d78;
  pvStack_c = ExceptionList;
  bVar2 = true;
  if (param_1 != 0) {
    uStack_10 = 0x32000242;
    uStack_14 = 0x71000043;
    param_1 = 0x98000441;
    iStack_4._0_1_ = 2;
    iStack_4._1_3_ = 0;
    ExceptionList = &pvStack_c;
    iVar1 = FUN_005479c0(&param_1,&uStack_14,&uStack_10,param_2);
    bVar2 = iVar1 != 0;
    iStack_4._0_1_ = 1;
    FUN_00619730();
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_00619730();
    iStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return bVar2;
}

