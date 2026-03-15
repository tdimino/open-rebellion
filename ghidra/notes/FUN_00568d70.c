
void * __thiscall FUN_00568d70(void *this,undefined4 *param_1)

{
  void *this_00;
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b3ab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(0x30);
  pvVar1 = (void *)0x0;
  local_4 = 0;
  if (this_00 != (void *)0x0) {
    pvVar1 = FUN_0058eec0(this_00,param_1);
  }
  local_4 = 0xffffffff;
  FUN_00568d50(this,(int)pvVar1);
  ExceptionList = local_c;
  return pvVar1;
}

