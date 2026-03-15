
void __fastcall FUN_0054f3f0(int *param_1)

{
  undefined4 local_28 [5];
  int local_14;
  undefined4 uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648798;
  local_c = ExceptionList;
  if (param_1[0x11] != 0) {
    ExceptionList = &local_c;
    FUN_004fd450(local_28,(int)(param_1 + 8));
    local_14 = param_1[0x11];
    local_4 = 0;
    uStack_10 = (**(code **)(*param_1 + 0x24))();
    FUN_004fd490(param_1 + 8,local_28);
    local_4 = 0xffffffff;
    FUN_004fd4d0(local_28);
  }
  ExceptionList = local_c;
  return;
}

