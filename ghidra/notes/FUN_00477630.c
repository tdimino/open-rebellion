
void * __thiscall FUN_00477630(void *this,undefined4 param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633082;
  local_c = ExceptionList;
  switch(param_1) {
  case 1:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x5c);
    local_4 = 0;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004ba580(pvVar1,*(undefined4 *)((int)this + 0x20),
                            *(undefined4 *)((int)this + 0x58),this);
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 2:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(100);
    local_4 = 1;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004b91a0(pvVar1,*(undefined4 *)((int)this + 0x20),
                            *(undefined4 *)((int)this + 0x58),this);
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 3:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x5c);
    local_4 = 4;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004b37f0(pvVar1,*(undefined4 *)((int)this + 0x20),
                            *(undefined4 *)((int)this + 0x58),this);
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 4:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x60);
    local_4 = 2;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004b6fb0(pvVar1,*(undefined4 *)((int)this + 0x20),
                            *(undefined4 *)((int)this + 0x58),this);
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 5:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x5c);
    local_4 = 5;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004b1720(pvVar1,*(undefined4 *)((int)this + 0x20),
                            *(undefined4 *)((int)this + 0x58),this);
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 6:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x5c);
    local_4 = 3;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004b4d50(pvVar1,*(undefined4 *)((int)this + 0x20),
                            *(undefined4 *)((int)this + 0x58),this);
      ExceptionList = local_c;
      return pvVar1;
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

