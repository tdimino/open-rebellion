
undefined4 __thiscall FUN_004f19e0(void *this,undefined4 param_1,undefined4 param_2,int param_3)

{
  void *pvVar1;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f038;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(&local_28,param_3);
  local_4 = 0;
  local_24 = 1;
  local_20 = 3;
  pvVar1 = (void *)FUN_00539fd0(this,1);
  FUN_004f06a0(pvVar1,&local_28);
  pvVar1 = (void *)FUN_00539fd0(this,2);
  FUN_004f06a0(pvVar1,&local_28);
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_28);
  ExceptionList = local_c;
  return 1;
}

