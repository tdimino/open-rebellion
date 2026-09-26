
int __thiscall FUN_00534b70(void *this,undefined4 param_1)

{
  int iVar1;
  int unaff_ESI;
  undefined4 unaff_retaddr;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00645a48;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(*(int *)this + 0x1c4))(param_1,&local_14);
  if ((iVar1 != 0) && (unaff_ESI == -1)) {
    iVar1 = (**(code **)(*(int *)this + 0x204))(unaff_retaddr);
  }
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_14;
  return iVar1;
}

