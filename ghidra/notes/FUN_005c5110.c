
void __thiscall FUN_005c5110(void *this,undefined4 param_1,uint param_2)

{
  void *this_00;
  uint *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653b6b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    puVar1 = (uint *)0x0;
  }
  else {
    puVar1 = (uint *)FUN_005fbd20(this_00,param_1,param_2,2);
  }
  local_4 = 0xffffffff;
  FUN_005ff2d0(this,puVar1,0,0);
  ExceptionList = local_c;
  return;
}

