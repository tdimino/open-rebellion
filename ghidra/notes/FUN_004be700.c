
undefined4 __thiscall FUN_004be700(void *this,int param_1)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639f4b;
  local_c = ExceptionList;
  uVar2 = 0;
  if (param_1 != 0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0x2c);
    local_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = FUN_004c8000(puVar1);
    }
    local_4 = 0xffffffff;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_004c8180(puVar1,param_1);
      FUN_005f4f10(this,(int)puVar1);
      uVar2 = 1;
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

