
undefined4 __cdecl FUN_00584fd0(int *param_1,int *param_2)

{
  int iVar1;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f3f8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  iVar1 = *(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1);
  while ((iVar1 == 0 && (*param_2 == 0))) {
    FUN_00584e60(param_1,local_70,param_2);
    iVar1 = *(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1);
  }
  *param_2 = 0;
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

