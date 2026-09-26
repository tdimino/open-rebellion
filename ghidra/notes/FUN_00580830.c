
undefined4 __thiscall FUN_00580830(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064e9b7;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  pvVar1 = (void *)FUN_00618b70(0x50);
  if (pvVar1 == (void *)0x0) {
    FUN_00619730();
  }
  local_4 = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    param_1 = pvVar1;
    pvVar1 = FUN_00580150(pvVar1,*(undefined4 *)((int)this + 0x30),*(undefined4 *)((int)this + 0x34)
                          ,*(undefined4 *)((int)this + 0x20));
  }
  if (pvVar1 != (void *)0x0) {
    if (*(int *)((int)this + 0x20) == 1) {
      param_1 = (void *)0xf300000f;
      local_4 = 2;
      FUN_004f26d0((void *)((int)pvVar1 + 0x3c),&param_1);
    }
    else {
      param_1 = (void *)0xf3000010;
      local_4 = 3;
      FUN_004f26d0((void *)((int)pvVar1 + 0x3c),&param_1);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_004fd350(pvVar1,*(undefined4 *)((int)this + 0x2c));
  }
  ExceptionList = local_c;
  return 1;
}

