
undefined4 __thiscall FUN_0042d860(void *this,void *param_1)

{
  int iVar1;
  undefined4 uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c7f8;
  local_c = ExceptionList;
  uVar2 = 0;
  ExceptionList = &local_c;
  FUN_004025b0(param_1,(uint *)&param_1);
  local_4 = 0;
  iVar1 = FUN_0060a860((void *)((int)this + 0x484),(uint)param_1 & 0xffffff);
  if (iVar1 != 0) {
    uVar2 = *(undefined4 *)(iVar1 + 0x20);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar2;
}

