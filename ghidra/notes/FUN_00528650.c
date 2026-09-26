
uint __thiscall FUN_00528650(void *this,int *param_1,undefined4 param_2,void *param_3)

{
  uint uVar1;
  int *piVar2;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00644620;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  uVar1 = (**(code **)(*(int *)this + 0x1e0))(param_1,param_3);
  if (uVar1 != 0) {
    if (*(int *)((int)param_3 + 4) == -1) {
      piVar2 = FUN_004f2d10(*param_1,(uint *)(param_1 + 2));
      uVar1 = (uint)(piVar2 != (int *)0x0);
    }
    if (uVar1 != 0) {
      ExceptionList = local_14;
      return uVar1;
    }
  }
  pvStack_c = (void *)0x1;
  FUN_00520580(param_3,(undefined4 *)&stack0xffffffe4);
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_14;
  return 0;
}

