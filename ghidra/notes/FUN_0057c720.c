
undefined4 __fastcall FUN_0057c720(void *param_1)

{
  int *piVar1;
  void *pvVar2;
  undefined1 local_2c [4];
  undefined4 local_28 [7];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064dd90;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004fd450(local_28,(int)param_1 + 0x20);
  local_4 = 0;
  piVar1 = (int *)thunk_FUN_00506e80();
  if (piVar1 != (int *)0x0) {
    pvVar2 = FUN_004fa150(param_1,local_2c);
    local_4._0_1_ = 1;
    (**(code **)(*piVar1 + 0x1d0))(pvVar2,(int)param_1 + 0x40,local_28);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = pvStack_c;
  return 0;
}

