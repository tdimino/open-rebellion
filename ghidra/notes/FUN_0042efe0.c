
void * __thiscall FUN_0042efe0(void *this,undefined4 param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ca47;
  local_c = ExceptionList;
  switch(param_1) {
  case 1:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x4c);
    local_4 = 0;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0047a430(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 3:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x5c);
    local_4 = 1;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0047a3c0(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 4:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x5c);
    local_4 = 2;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0047a000(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 5:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x5c);
    local_4 = 3;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_00479f70(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 6:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x4c);
    local_4 = 4;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_00479a60(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

