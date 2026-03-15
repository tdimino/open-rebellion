
void __thiscall FUN_005d59e0(void *this,undefined4 param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006547db;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x70);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005da150(pvVar1,*(undefined4 *)((int)this + 0x4c),param_1);
  }
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_005f59f0((void *)(DAT_006bcbd4 + 0x5cc),(int)pvVar1 + 0x1c);
  }
  ExceptionList = local_c;
  return;
}

