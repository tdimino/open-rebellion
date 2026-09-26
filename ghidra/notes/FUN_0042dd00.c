
void __thiscall FUN_0042dd00(void *this,int param_1)

{
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  int iVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c852;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  for (pvVar2 = (void *)FUN_0051cad0(0x14); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    if ((*(uint *)((int)pvVar2 + 0x2c) < 0x14) || (0x1b < *(uint *)((int)pvVar2 + 0x2c))) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      pvVar3 = (void *)FUN_00618b70(0x60);
      local_4 = 0;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_00473830(pvVar3,pvVar2);
      }
      local_4 = 0xffffffff;
      if (((pvVar3 != (void *)0x0) &&
          (FUN_005f4f10(this,(int)pvVar3), *(int *)((int)pvVar3 + 0x28) == param_1)) &&
         (*(int *)((int)this + 8) < *(int *)((int)pvVar3 + 0x24))) {
        *(int *)((int)this + 8) = *(int *)((int)pvVar3 + 0x24);
      }
    }
  }
  for (pvVar2 = (void *)FUN_0051cad0(0x1c); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    if ((*(uint *)((int)pvVar2 + 0x2c) < 0x1c) || (0x1f < *(uint *)((int)pvVar2 + 0x2c))) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      pvVar3 = (void *)FUN_00618b70(0x50);
      local_4 = 1;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_00473570(pvVar3,pvVar2);
      }
      local_4 = 0xffffffff;
      if (((pvVar3 != (void *)0x0) &&
          (FUN_005f4f10(this,(int)pvVar3), *(int *)((int)pvVar3 + 0x28) == param_1)) &&
         (*(int *)((int)this + 8) < *(int *)((int)pvVar3 + 0x24))) {
        *(int *)((int)this + 8) = *(int *)((int)pvVar3 + 0x24);
      }
    }
  }
  for (pvVar2 = (void *)FUN_0051cad0(0x10); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    if ((*(uint *)((int)pvVar2 + 0x2c) < 0x10) || (0x13 < *(uint *)((int)pvVar2 + 0x2c))) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      pvVar3 = (void *)FUN_00618b70(0x44);
      local_4 = 2;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_00473400(pvVar3,pvVar2);
      }
      local_4 = 0xffffffff;
      if (((pvVar3 != (void *)0x0) &&
          (FUN_005f4f10(this,(int)pvVar3), *(int *)((int)pvVar3 + 0x28) == param_1)) &&
         (*(int *)((int)this + 0xc) < *(int *)((int)pvVar3 + 0x24))) {
        *(int *)((int)this + 0xc) = *(int *)((int)pvVar3 + 0x24);
      }
    }
  }
  for (pvVar2 = (void *)FUN_0051cad0(0x22); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    if ((*(uint *)((int)pvVar2 + 0x2c) < 0x22) || (0x27 < *(uint *)((int)pvVar2 + 0x2c))) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      pvVar3 = (void *)FUN_00618b70(0x38);
      local_4 = 3;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_004731c0(pvVar3,pvVar2);
      }
      local_4 = 0xffffffff;
      if (pvVar3 != (void *)0x0) {
        FUN_005f4f10(this,(int)pvVar3);
        if (*(int *)((int)this + 0x10) < *(int *)((int)pvVar3 + 0x24)) {
          *(int *)((int)this + 0x10) = *(int *)((int)pvVar3 + 0x24);
        }
      }
    }
  }
  for (pvVar2 = (void *)FUN_0051cad0(0x28); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    if ((*(uint *)((int)pvVar2 + 0x2c) < 0x28) || (0x2b < *(uint *)((int)pvVar2 + 0x2c))) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      pvVar3 = (void *)FUN_00618b70(0x34);
      local_4 = 4;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_00472fd0(pvVar3,pvVar2);
      }
      local_4 = 0xffffffff;
      if (pvVar3 != (void *)0x0) {
        FUN_005f4f10(this,(int)pvVar3);
        if (*(int *)((int)this + 0x10) < *(int *)((int)pvVar3 + 0x24)) {
          *(int *)((int)this + 0x10) = *(int *)((int)pvVar3 + 0x24);
        }
      }
    }
  }
  for (pvVar2 = (void *)FUN_0051cad0(0x3c); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    if ((*(uint *)((int)pvVar2 + 0x2c) < 0x3c) || (0x3f < *(uint *)((int)pvVar2 + 0x2c))) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      pvVar3 = (void *)FUN_00618b70(0x40);
      local_4 = 5;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_00472be0(pvVar3,pvVar2);
      }
      local_4 = 0xffffffff;
      if (pvVar3 != (void *)0x0) {
        FUN_005f4f10(this,(int)pvVar3);
      }
    }
  }
  iVar4 = FUN_0051cad0(0x38);
  do {
    if (iVar4 == 0) {
      FUN_0042e210(this,*(int *)((int)this + 0x18),param_1);
      FUN_0042e330(this,*(int *)((int)this + 0x1c),param_1);
      FUN_0042e3b0(this,*(undefined4 *)((int)this + 0x20));
      ExceptionList = local_c;
      return;
    }
    if ((*(uint *)(iVar4 + 0x2c) < 0x38) || (0x3b < *(uint *)(iVar4 + 0x2c))) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (*(int *)(iVar4 + 0x44) == 0) {
        if (param_1 == 1) goto LAB_0042e140;
      }
      else if (param_1 == 2) {
LAB_0042e140:
        *(int *)((int)this + 0x14) = *(int *)((int)this + 0x14) + 1;
      }
    }
    iVar4 = *(int *)(iVar4 + 0x10);
  } while( true );
}

