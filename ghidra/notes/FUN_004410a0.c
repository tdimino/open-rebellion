
void FUN_004410a0(void *param_1,int *param_2)

{
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062e938;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_0049ecf0(local_2c,(int)param_2);
  local_4 = 0;
  if (local_10 != 0) {
    FUN_0049ed60(local_2c,param_1);
    (**(code **)(*param_2 + 0x24))(param_1);
  }
  local_4 = 0xffffffff;
  FUN_0049ed50(local_2c);
  ExceptionList = pvStack_c;
  return;
}

