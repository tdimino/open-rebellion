
void * __thiscall FUN_004bea20(void *this,undefined4 param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639ffa;
  local_c = ExceptionList;
  switch(param_1) {
  case 1:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x78);
    local_4 = 2;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004da950(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 2:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x80);
    local_4 = 0;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004e1e50(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 3:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x7c);
    local_4 = 3;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004d6b20(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 4:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x60);
    local_4 = 6;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004d2220(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 5:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x88);
    local_4 = 7;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004cfb20(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 6:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x8c);
    local_4 = 1;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004dd460(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 7:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x7c);
    local_4 = 4;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004d2ef0(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 8:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x58);
    local_4 = 8;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004cf410(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 9:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x58);
    local_4 = 9;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004cf0a0(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 10:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x84);
    local_4 = 10;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004cc6b0(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0xb:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x88);
    local_4 = 0xb;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004c8820(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0xc:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x60);
    local_4 = 0xc;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004c8260(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0xd:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x70);
    local_4 = 0xd;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004bf0d0(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0xe:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x4c);
    local_4 = 5;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004d2810(pvVar1,*(undefined4 *)((int)this + 0xc));
      ExceptionList = local_c;
      return pvVar1;
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

