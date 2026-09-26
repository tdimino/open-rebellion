
void * __thiscall FUN_00415cd0(void *this,void *param_1,int param_2,int param_3)

{
  char *pcVar1;
  int iVar2;
  undefined4 local_9c [3];
  undefined4 local_90 [10];
  int local_68 [3];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ac88;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00411730(local_90);
  local_4 = 1;
  FUN_00415ad0(this,local_9c,param_2,param_3);
  local_4._0_1_ = 2;
  FUN_00616620(local_68,1);
  local_4 = CONCAT31(local_4._1_3_,3);
  pcVar1 = (char *)FUN_00583c40((int)local_9c);
  FUN_005f4a60(local_68,pcVar1);
  iVar2 = FUN_005f49b0();
  if (iVar2 == 0) {
    FUN_00411900(local_90,local_68);
  }
  FUN_005f4bc0(local_68);
  FUN_004117c0(param_1,local_90);
  local_4._0_1_ = 2;
  FUN_00616810((int)auStack_5c);
  FUN_00615440(auStack_5c);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_9c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004118f0(local_90);
  ExceptionList = pvStack_c;
  return param_1;
}

