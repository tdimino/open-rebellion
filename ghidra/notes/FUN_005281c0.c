
int __thiscall FUN_005281c0(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00644550;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = (**(code **)(*(int *)this + 0x1c4))(param_1,param_3);
  if (iVar1 == 0) {
    pvStack_c = (void *)0x1;
    FUN_00520580(param_3,(undefined4 *)&stack0xffffffe4);
    pvStack_c = (void *)0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_14;
  return iVar1;
}

