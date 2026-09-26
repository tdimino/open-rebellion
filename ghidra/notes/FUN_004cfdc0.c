
void __thiscall FUN_004cfdc0(void *this,void *param_1)

{
  void *unaff_EDI;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063bcc8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_004e50c0(this,param_1);
  FUN_005f4db0(param_1,(int)this + 0x38);
  FUN_005f4db0(param_1,(int)this + 0x3c);
  FUN_005f4df0(param_1,(int)this + 0x40);
  FUN_005f4db0(param_1,(int)this + 0x48);
  FUN_005f4db0(param_1,(int)this + 0x4c);
  FUN_005f4db0(param_1,(int)this + 0x50);
  FUN_005f4db0(param_1,(int)this + 0x54);
  FUN_005f4db0(param_1,(int)this + 0x44);
  (**(code **)(*(int *)((int)this + 0x68) + 0xc))(param_1);
  FUN_004ecea0((void *)((int)this + 0x58),param_1);
  FUN_004ecea0((void *)((int)this + 0x5c),param_1);
  FUN_004ecea0((void *)((int)this + 0x60),param_1);
  FUN_004ecea0((void *)((int)this + 100),param_1);
  (**(code **)(*(int *)((int)this + 0x78) + 0x18))(param_1);
  (**(code **)(*(int *)((int)this + 0x80) + 0x18))(param_1);
  local_10 = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_EDI;
  return;
}

