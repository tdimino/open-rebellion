
void * __thiscall FUN_00435bc0(void *this,undefined4 param_1,int param_2)

{
  undefined1 uVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d4cd;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00536eb0((undefined4 *)((int)this + 0x24));
  *(undefined4 *)((int)this + 0x1c) = param_1;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x20) = 1;
  *(undefined4 *)this = 0;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 8) = 1;
  if (param_2 == 1) {
    pvVar2 = (void *)FUN_00618b70(0xc4);
    local_4._0_1_ = 1;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_004861b0(pvVar2,1);
    }
    *(void **)((int)this + 0xc) = pvVar2;
    *(void **)((int)this + 0x10) = pvVar2;
    local_4._0_1_ = 0;
    uVar1 = (undefined1)local_4;
    local_4._0_1_ = 0;
    if (*(int *)((int)this + 0x1c) == 1) {
      pvVar2 = (void *)FUN_00618b70(0x42c);
      local_4 = CONCAT31(local_4._1_3_,2);
      if (pvVar2 != (void *)0x0) {
        pvVar2 = FUN_004857a0(pvVar2,2);
        *(void **)((int)this + 0x14) = pvVar2;
        ExceptionList = local_c;
        return this;
      }
    }
    else {
      local_4._0_1_ = uVar1;
      pvVar2 = (void *)FUN_00618b70(0x60);
      local_4 = CONCAT31(local_4._1_3_,3);
      if (pvVar2 != (void *)0x0) {
        pvVar2 = FUN_00484e70(pvVar2,2);
        *(void **)((int)this + 0x14) = pvVar2;
        ExceptionList = local_c;
        return this;
      }
    }
    *(undefined4 *)((int)this + 0x14) = 0;
    ExceptionList = local_c;
    return this;
  }
  pvVar2 = (void *)FUN_00618b70(0xc4);
  local_4._0_1_ = 4;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_004861b0(pvVar2,2);
  }
  *(void **)((int)this + 0xc) = pvVar2;
  *(void **)((int)this + 0x14) = pvVar2;
  local_4._0_1_ = 0;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 0;
  if (*(int *)((int)this + 0x1c) == 1) {
    pvVar2 = (void *)FUN_00618b70(0x42c);
    local_4 = CONCAT31(local_4._1_3_,5);
    if (pvVar2 != (void *)0x0) {
      pvVar2 = FUN_004857a0(pvVar2,1);
      goto LAB_00435d1b;
    }
  }
  else {
    local_4._0_1_ = uVar1;
    pvVar2 = (void *)FUN_00618b70(0x60);
    local_4 = CONCAT31(local_4._1_3_,6);
    if (pvVar2 != (void *)0x0) {
      pvVar2 = FUN_00484e70(pvVar2,1);
      goto LAB_00435d1b;
    }
  }
  pvVar2 = (void *)0x0;
LAB_00435d1b:
  *(void **)((int)this + 0x10) = pvVar2;
  ExceptionList = local_c;
  return this;
}

