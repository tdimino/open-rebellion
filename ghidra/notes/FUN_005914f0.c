
undefined4 __thiscall FUN_005914f0(void *this,int *param_1,int *param_2)

{
  int *this_00;
  undefined4 *puVar1;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650678;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  FUN_00595640(this,param_1,param_2);
  puVar1 = (undefined4 *)((int)this + 0x5c);
  this_00 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(this_00,puVar1);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

