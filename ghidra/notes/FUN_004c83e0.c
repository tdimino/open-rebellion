
void __thiscall FUN_004c83e0(void *this,void *param_1)

{
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063aed8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_004e50c0(this,param_1);
  FUN_005f4db0(param_1,(int)this + 0x38);
  FUN_005f4db0(param_1,(int)this + 0x3c);
  FUN_005f4db0(param_1,(int)this + 0x40);
  FUN_005f4db0(param_1,(int)this + 0x44);
  FUN_005f4db0(param_1,(int)this + 0x48);
  FUN_005f4db0(param_1,(int)this + 0x4c);
  (**(code **)(*(int *)((int)this + 0x50) + 0xc))(param_1);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_10;
  return;
}

