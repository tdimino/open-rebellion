
undefined4 __thiscall FUN_005916c0(void *this,int *param_1,int *param_2)

{
  undefined1 local_70 [20];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006506b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  FUN_00595980(this,param_1,param_2);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

