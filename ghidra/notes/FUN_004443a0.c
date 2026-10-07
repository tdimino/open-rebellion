
void __thiscall FUN_004443a0(int *param_1,uint param_2)

{
  bool bVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062eda8;
  pvStack_c = ExceptionList;
  if ((param_2 & 0xffff) == 0x65) {
    ExceptionList = &pvStack_c;
    (**(code **)(*param_1 + 0x30))();
  }
  else if ((param_2 & 0xffff) == 0x66) {
    ExceptionList = &pvStack_c;
    FUN_0042dbe0(&uStack_18);
    iStack_4 = 0;
    FUN_004ece30(&param_2);
    iStack_4._0_1_ = 1;
    if (param_1[0x49] != param_2) {
      uVar2 = (uint)param_1[0x49] >> 0x18;
      uStack_14 = 0xf1;
      uStack_10 = 0xf2;
      if ((uVar2 < 0xf1) || (0xf1 < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        (**(code **)(*param_1 + 0x30))();
        puVar3 = (undefined4 *)FUN_004ece30(&uStack_1c);
        iStack_4._0_1_ = 2;
        FUN_0041d6b0(puVar3,param_1 + 0x4a);
        iStack_4._0_1_ = 1;
        FUN_00619730();
      }
    }
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_00619730();
    iStack_4 = 0xffffffff;
    FUN_00619730();
    ExceptionList = pvStack_c;
    return;
  }
  ExceptionList = pvStack_c;
  return;
}

