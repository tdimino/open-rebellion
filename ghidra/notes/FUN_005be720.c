
void __thiscall FUN_005be720(void *this,void *param_1)

{
  undefined4 local_1c [2];
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653760;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005acf80(this,param_1);
  FUN_005f5fa0(local_1c);
  local_4 = 0;
  FUN_005ce6c0((void *)((int)this + 0x908),param_1);
  FUN_005ce7b0((void *)((int)this + 0x908),local_1c);
  FUN_005ce6c0((void *)((int)this + 0x914),param_1);
  FUN_005ce7b0((void *)((int)this + 0x914),local_1c);
  FUN_005ce6c0((void *)((int)this + 0x920),param_1);
  FUN_005ce7b0((void *)((int)this + 0x920),local_1c);
  FUN_005ce6c0((void *)((int)this + 0x92c),param_1);
  FUN_005ce7b0((void *)((int)this + 0x92c),local_1c);
  FUN_005ce6c0((void *)((int)this + 0x938),param_1);
  FUN_005ce7b0((void *)((int)this + 0x938),local_1c);
  FUN_005ce6c0((void *)((int)this + 0x944),param_1);
  FUN_005ce7b0((void *)((int)this + 0x944),local_1c);
  FUN_005f5fa0(local_14);
  local_4._0_1_ = 1;
  FUN_005f6150(local_14,param_1);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f5fe0(local_14);
  local_4 = 0xffffffff;
  FUN_005f5fe0(local_1c);
  ExceptionList = local_c;
  return;
}

