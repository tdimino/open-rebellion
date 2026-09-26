
undefined4 * __thiscall FUN_005a4a30(void *this,undefined4 param_1)

{
  undefined4 *puVar1;
  void *this_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006525b6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar1);
    }
  }
  local_4 = 0xffffffff;
  this_00 = (void *)FUN_005a6470(DAT_006bc470,*(uint *)((int)this + 0x58));
  if (this_00 == (void *)0x0) {
    puVar1 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 1;
    if (puVar1 == (undefined4 *)0x0) {
      ExceptionList = local_c;
      return (undefined4 *)0x0;
    }
    puVar1 = FUN_005a0c80(puVar1);
    ExceptionList = local_c;
    return puVar1;
  }
  FUN_005cb160(this_00,param_1);
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

