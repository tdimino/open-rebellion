
void __thiscall FUN_004cf150(void *this,void *param_1)

{
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063bae8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_004e5070(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x38);
  FUN_005f4d90(param_1,(int)this + 0x3c);
  FUN_005f4d90(param_1,(int)this + 0x40);
  FUN_005f4d90(param_1,(int)this + 0x44);
  (**(code **)(*(int *)((int)this + 0x48) + 8))(param_1);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_10;
  return;
}

