
void __thiscall FUN_004ac730(void *this,uint param_1)

{
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006385c8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_0041cdf0(param_1,(int)this + 0x11c,(uint *)&local_10,0);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

