
void __thiscall FUN_004cfcb0(void *this,void *param_1)

{
  void *unaff_EDI;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063bca8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_004e5070(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x38);
  FUN_005f4d90(param_1,(int)this + 0x3c);
  FUN_005f4dd0(param_1,(int)this + 0x40);
  FUN_005f4d90(param_1,(int)this + 0x48);
  FUN_005f4d90(param_1,(int)this + 0x4c);
  FUN_005f4d90(param_1,(int)this + 0x50);
  FUN_005f4d90(param_1,(int)this + 0x54);
  FUN_005f4d90(param_1,(int)this + 0x44);
  (**(code **)(*(int *)((int)this + 0x68) + 8))(param_1);
  FUN_004ece90((void *)((int)this + 0x58),param_1);
  FUN_004ece90((void *)((int)this + 0x5c),param_1);
  FUN_004ece90((void *)((int)this + 0x60),param_1);
  FUN_004ece90((void *)((int)this + 100),param_1);
  (**(code **)(*(int *)((int)this + 0x78) + 0x10))(param_1);
  (**(code **)(*(int *)((int)this + 0x80) + 0x10))(param_1);
  local_10 = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_EDI;
  return;
}

