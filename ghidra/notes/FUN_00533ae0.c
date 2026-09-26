
int __thiscall FUN_00533ae0(void *this,int param_1,void *param_2)

{
  int iVar1;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00645900;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = (**(code **)(*(int *)this + 0x6c))(param_1,param_2);
  if ((iVar1 != 0) && (iVar1 = FUN_004f9860(this,param_1,param_2), iVar1 != 0)) {
    ExceptionList = local_14;
    return iVar1;
  }
  pvStack_c = (void *)0x1;
  FUN_00520580(param_2,(undefined4 *)&stack0xffffffe4);
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_14;
  return 0;
}

