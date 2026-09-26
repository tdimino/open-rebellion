
undefined4 __thiscall FUN_005905c0(void *this,int *param_1,int *param_2)

{
  int *this_00;
  int *piVar1;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650538;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  FUN_00593f90(this,param_1,param_2);
  piVar1 = (int *)((int)this + 0x98);
  this_00 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617760(this_00,piVar1);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

