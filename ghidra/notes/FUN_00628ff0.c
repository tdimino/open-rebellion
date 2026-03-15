
void __thiscall FUN_00628ff0(void *this,void *param_1)

{
  void *this_00;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00657398;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  this_00 = (void *)FUN_00506f30(*(int *)((int)this + 0x20));
  if (this_00 != (void *)0x0) {
    FUN_005305e0(this_00,*(int *)((int)this + 0x2c));
  }
  ExceptionList = local_c;
  return;
}

