
void __thiscall FUN_004c5880(void *this,void *param_1,void *param_2,undefined4 param_3)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a92b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(200);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_004c5e20(puVar1);
  }
  local_4 = 0xffffffff;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_004c60d0(puVar1,param_1,param_2);
    puVar1[0xd] = param_3;
    FUN_004f57b0(this,puVar1);
  }
  ExceptionList = local_c;
  return;
}

