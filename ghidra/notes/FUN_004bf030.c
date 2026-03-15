
void __thiscall FUN_004bf030(void *this,int *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a03b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x2c);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_004e4d40(puVar1);
  }
  local_4 = 0xffffffff;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_004e4f20(puVar1,param_1);
    FUN_005f5440(this,puVar1 + 3);
  }
  ExceptionList = local_c;
  return;
}

