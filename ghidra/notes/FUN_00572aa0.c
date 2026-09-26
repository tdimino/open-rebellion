
undefined4 __thiscall FUN_00572aa0(void *this,undefined4 param_1,int param_2)

{
  void *pvVar1;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064c488;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(&local_28,param_2);
  local_4 = 0;
  local_24 = 1;
  local_20 = 3;
  pvVar1 = (void *)FUN_00539fd0(this,1);
  FUN_00572950(pvVar1,&local_28);
  pvVar1 = (void *)FUN_00539fd0(this,2);
  FUN_00572950(pvVar1,&local_28);
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_28);
  ExceptionList = local_c;
  return 1;
}

