
void __fastcall FUN_004883d0(void *param_1)

{
  uint local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634ad0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = 0;
  FUN_004ece30(&local_18);
  local_4._0_1_ = 1;
  FUN_004888e0((int)param_1);
  FUN_00486fb0(param_1,5,(int)local_14,&local_18,0);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return;
}

