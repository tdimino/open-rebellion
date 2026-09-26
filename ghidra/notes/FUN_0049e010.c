
void * __thiscall FUN_0049e010(void *this,undefined4 param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006371f1;
  local_c = ExceptionList;
  switch(param_1) {
  case 0x14:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x58);
    local_4 = 2;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004c6ad0(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x15:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x5c);
    local_4 = 1;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004c7340(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x17:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x58);
    local_4 = 0;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0049e1c0(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

