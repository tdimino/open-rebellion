
void __thiscall FUN_005be610(void *this,void *param_1)

{
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653738;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005acaf0(this,param_1);
  FUN_005f5fa0(local_14);
  local_4 = 0;
  FUN_005ce630((void *)((int)this + 0x908),param_1);
  FUN_005ce7b0((void *)((int)this + 0x908),local_14);
  FUN_005ce630((void *)((int)this + 0x914),param_1);
  FUN_005ce7b0((void *)((int)this + 0x914),local_14);
  FUN_005ce630((void *)((int)this + 0x920),param_1);
  FUN_005ce7b0((void *)((int)this + 0x920),local_14);
  FUN_005ce630((void *)((int)this + 0x92c),param_1);
  FUN_005ce7b0((void *)((int)this + 0x92c),local_14);
  FUN_005ce630((void *)((int)this + 0x938),param_1);
  FUN_005ce7b0((void *)((int)this + 0x938),local_14);
  FUN_005ce630((void *)((int)this + 0x944),param_1);
  FUN_005ce7b0((void *)((int)this + 0x944),local_14);
  FUN_005f6160(local_14,param_1);
  local_4 = 0xffffffff;
  FUN_005f5fe0(local_14);
  ExceptionList = local_c;
  return;
}

