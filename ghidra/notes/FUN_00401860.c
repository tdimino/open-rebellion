
void * __cdecl FUN_00401860(void *param_1,int param_2,int param_3)

{
  void *pvVar1;
  undefined4 local_5c [10];
  undefined4 local_34 [10];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629177;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00411730(local_5c);
  local_4 = 1;
  if (DAT_006be3b8 != (void *)0x0) {
    pvVar1 = FUN_00415cd0(DAT_006be3b8,local_34,param_2,param_3);
    local_4._0_1_ = 2;
    FUN_00411860(local_5c,pvVar1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_004118f0(local_34);
  }
  FUN_004117c0(param_1,local_5c);
  local_4 = local_4 & 0xffffff00;
  FUN_004118f0(local_5c);
  ExceptionList = pvStack_c;
  return param_1;
}

