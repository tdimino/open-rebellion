
void __fastcall FUN_005f8740(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00655b3a;
  pvStack_c = ExceptionList;
  local_4 = 9;
  ExceptionList = &pvStack_c;
  FUN_005ae1e0();
  if (DAT_006be3b0 != (undefined4 *)0x0) {
    (**(code **)*DAT_006be3b0)(1);
    DAT_006be3b0 = (undefined4 *)0x0;
  }
  DAT_006be3a8 = 0;
  DAT_006be3a4 = 0;
  local_4._0_1_ = 8;
  FUN_005fb130(param_1 + 0x91f);
  local_4._0_1_ = 7;
  FUN_00618d20(param_1 + 0x6c,0x20c,0x11,FUN_005fb270);
  local_4._0_1_ = 6;
  FUN_005f2f40(param_1 + 0x69);
  local_4._0_1_ = 5;
  FUN_005f2f40(param_1 + 0x66);
  local_4._0_1_ = 4;
  FUN_005f2ff0(param_1 + 0x26);
  local_4._0_1_ = 3;
  FUN_005f5300(param_1 + 0x21);
  local_4._0_1_ = 2;
  FUN_005f5300(param_1 + 0x1e);
  local_4._0_1_ = 1;
  FUN_005f5300(param_1 + 6);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f5300(param_1 + 3);
  local_4 = 0xffffffff;
  FUN_005f5300(param_1);
  ExceptionList = pvStack_c;
  return;
}

