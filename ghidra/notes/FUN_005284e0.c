
int __thiscall FUN_005284e0(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  void *unaff_EBP;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006445e0;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = (**(code **)(*(int *)this + 0x1d8))(param_1,param_2,param_3);
  if (iVar1 == 0) {
    local_10 = 1;
    FUN_00520580(param_3,(undefined4 *)&stack0xffffffe0);
    local_10 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = unaff_EBP;
  return iVar1;
}

