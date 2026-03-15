
void __thiscall FUN_00430cc0(void *this,void *param_1,void *param_2,undefined4 param_3)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cd3b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x1d8);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00431270(puVar1);
  }
  local_4 = 0xffffffff;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_00431550(puVar1,param_1,param_2);
    puVar1[0x11] = param_3;
    FUN_004f57b0(this,puVar1);
  }
  ExceptionList = local_c;
  return;
}

