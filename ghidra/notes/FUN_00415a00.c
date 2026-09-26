
void * __thiscall FUN_00415a00(void *this,void *param_1,int param_2,int param_3)

{
  void *pvVar1;
  undefined4 local_4c [3];
  undefined4 local_40 [3];
  undefined4 local_34 [10];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062abef;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_4c);
  local_4 = 1;
  FUN_00415cd0(this,local_34,param_2,param_3);
  local_4._0_1_ = 2;
  pvVar1 = FUN_00408d90(local_34,local_40);
  local_4._0_1_ = 3;
  FUN_005f3090(local_4c,(int)pvVar1);
  local_4._0_1_ = 2;
  FUN_005f2ff0(local_40);
  FUN_005f2f90(param_1,(int)local_4c);
  local_4._0_1_ = 1;
  FUN_004118f0(local_34);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_4c);
  ExceptionList = pvStack_c;
  return param_1;
}

