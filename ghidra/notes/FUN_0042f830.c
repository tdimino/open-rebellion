
void * __thiscall FUN_0042f830(void *this,undefined4 param_1,int param_2)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cb5b;
  local_c = ExceptionList;
  switch(param_1) {
  case 0x51:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x70);
    local_4 = 4;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_00481460(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x52:
    if (param_2 == 6) {
      ExceptionList = &local_c;
      pvVar1 = (void *)FUN_00618b70(0x74);
      local_4 = 5;
      if (pvVar1 != (void *)0x0) {
        pvVar1 = FUN_00480d10(pvVar1,*(undefined4 *)((int)this + 0xc),
                              *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
        ExceptionList = local_c;
        return pvVar1;
      }
    }
    else {
      ExceptionList = &local_c;
      pvVar1 = (void *)FUN_00618b70(0x7c);
      local_4 = 6;
      if (pvVar1 != (void *)0x0) {
        pvVar1 = FUN_00480730(pvVar1,*(undefined4 *)((int)this + 0xc),
                              *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
        ExceptionList = local_c;
        return pvVar1;
      }
    }
    break;
  case 0x53:
    if (param_2 == 1) {
      ExceptionList = &local_c;
      pvVar1 = (void *)FUN_00618b70(0x70);
      local_4 = 0xe;
      if (pvVar1 != (void *)0x0) {
        pvVar1 = FUN_0047bba0(pvVar1,*(undefined4 *)((int)this + 0xc),
                              *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
        ExceptionList = local_c;
        return pvVar1;
      }
    }
    else if (param_2 == 2) {
      ExceptionList = &local_c;
      pvVar1 = (void *)FUN_00618b70(0x70);
      local_4 = 0xf;
      if (pvVar1 != (void *)0x0) {
        pvVar1 = FUN_0047c170(pvVar1,*(undefined4 *)((int)this + 0xc),
                              *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
        ExceptionList = local_c;
        return pvVar1;
      }
    }
    else if (param_2 == 3) {
      ExceptionList = &local_c;
      pvVar1 = (void *)FUN_00618b70(0x70);
      local_4 = 0x10;
      if (pvVar1 != (void *)0x0) {
        pvVar1 = FUN_0047c750(pvVar1,*(undefined4 *)((int)this + 0xc),
                              *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
        ExceptionList = local_c;
        return pvVar1;
      }
    }
    break;
  case 0x54:
    if (param_2 == 5) {
      ExceptionList = &local_c;
      pvVar1 = (void *)FUN_00618b70(0x74);
      local_4 = 8;
      if (pvVar1 != (void *)0x0) {
        pvVar1 = FUN_0047f550(pvVar1,*(undefined4 *)((int)this + 0xc),
                              *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
        ExceptionList = local_c;
        return pvVar1;
      }
    }
    else {
      ExceptionList = &local_c;
      pvVar1 = (void *)FUN_00618b70(0x7c);
      local_4 = 9;
      if (pvVar1 != (void *)0x0) {
        pvVar1 = FUN_0047f190(pvVar1,*(undefined4 *)((int)this + 0xc),
                              *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
        ExceptionList = local_c;
        return pvVar1;
      }
    }
    break;
  case 0x55:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x70);
    local_4 = 0xb;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0047d9a0(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x56:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x70);
    local_4 = 7;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0047fe50(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x57:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x70);
    local_4 = 0xd;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0047cda0(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x61:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x70);
    local_4 = 10;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0047e130(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x62:
    if (param_2 != 4) {
      ExceptionList = &local_c;
      pvVar1 = (void *)FUN_00618b70(0x70);
      local_4 = 1;
      if (pvVar1 == (void *)0x0) {
        ExceptionList = local_c;
        return (void *)0x0;
      }
      pvVar1 = FUN_00483140(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x80);
    local_4 = 0;
    goto joined_r0x0042f8f1;
  case 99:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x80);
    local_4 = 2;
joined_r0x0042f8f1:
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_00482b30(pvVar1,*(int *)((int)this + 0xc),*(undefined4 *)((int)this + 0x14),
                            *(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x69:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x7c);
    local_4 = 0xc;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_0047d380(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
    break;
  case 0x6a:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x70);
    local_4 = 3;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_00482020(pvVar1,*(undefined4 *)((int)this + 0xc),
                            *(undefined4 *)((int)this + 0x14),*(undefined4 *)((int)this + 0x10));
      ExceptionList = local_c;
      return pvVar1;
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

