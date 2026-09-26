
uint __thiscall FUN_00552d10(void *this,int *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648ed2;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x228);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00550620(puVar1);
  }
  local_4 = 0xffffffff;
  uVar2 = 0;
  if (puVar1 != (undefined4 *)0x0) {
    if (puVar1[7] == 0) {
      if (puVar1 != (undefined4 *)0x0) {
        (**(code **)*puVar1)(1);
      }
      uVar2 = 0;
    }
    else {
      local_4 = 0xffffffff;
      uVar2 = FUN_00550700(puVar1,*(undefined4 *)((int)this + 0x14),
                           *(undefined4 *)((int)this + 0x18),*(undefined4 *)((int)this + 0x1c),
                           (undefined4 *)((int)this + 0x20),param_1);
      if (uVar2 != 0) {
        FUN_004f57b0(this,puVar1);
      }
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

