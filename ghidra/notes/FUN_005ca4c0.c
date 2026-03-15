
void * __thiscall FUN_005ca4c0(void *this,int param_1)

{
  void *this_00;
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006541ab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(0x20);
  pvVar1 = (void *)0x0;
  local_4 = 0;
  if (this_00 != (void *)0x0) {
    pvVar1 = FUN_005ca350(this_00,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005f5440(this,pvVar1);
  ExceptionList = local_c;
  return pvVar1;
}

