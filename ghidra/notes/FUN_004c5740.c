
undefined4 __thiscall FUN_004c5740(void *this,void *param_1,undefined4 param_2)

{
  void *this_00;
  int iVar1;
  int *piVar2;
  undefined4 local_18;
  void *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a913;
  local_c = ExceptionList;
  local_18 = 0;
  if (*(int *)((int)this + 0x10) == 0) {
    ExceptionList = &local_c;
    (**(code **)(*(int *)this + 4))();
    piVar2 = FUN_004f3900(*(int *)((int)this + 8));
    *(int **)((int)this + 0x10) = piVar2;
  }
  else {
    ExceptionList = &local_c;
    local_14 = (void *)FUN_00618b70(200);
    local_4 = 0;
    if (local_14 == (void *)0x0) {
      this_00 = (void *)0x0;
    }
    else {
      this_00 = FUN_004c5ec0(local_14,*(undefined4 *)((int)this + 8));
    }
    local_4 = 0xffffffff;
    *(void **)((int)this + 0xc) = this_00;
    if (this_00 != (void *)0x0) {
      FUN_004c5fc0(this_00,*(void **)((int)this + 0x10),param_1);
      *(undefined4 *)(*(int *)((int)this + 0xc) + 0x34) = param_2;
      FUN_004f57b0(this,*(void **)((int)this + 0xc));
    }
    local_14 = (void *)0x80;
    local_10 = 0x90;
    local_4 = 1;
    iVar1 = FUN_004f6010(*(void **)((int)this + 0x10),(uint *)&local_14,1);
    local_4 = 0xffffffff;
    FUN_00619730();
    *(int *)((int)this + 0x10) = iVar1;
    if (iVar1 == 0) {
      local_18 = 1;
      *(undefined4 *)((int)this + 0xc) = 0;
    }
  }
  ExceptionList = local_c;
  return local_18;
}

