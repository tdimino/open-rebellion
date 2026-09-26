
uint __thiscall FUN_00529ba0(void *this,undefined4 param_1)

{
  uint uVar1;
  int unaff_ESI;
  undefined4 unaff_retaddr;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00644878;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  uVar1 = (**(code **)(*(int *)this + 0x1d0))(param_1,&local_14);
  if ((uVar1 != 0) && (unaff_ESI == -1)) {
    uVar1 = FUN_0052a2c0(this,unaff_retaddr);
  }
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_14;
  return uVar1;
}

