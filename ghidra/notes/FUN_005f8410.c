
undefined4 * __fastcall FUN_005f8410(undefined4 *param_1)

{
  DWORD DVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655a9a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f52c0(param_1);
  local_4 = 0;
  FUN_005f52c0(param_1 + 3);
  local_4._0_1_ = 1;
  FUN_005f52c0(param_1 + 6);
  local_4._0_1_ = 2;
  FUN_005f52c0(param_1 + 0x1e);
  local_4._0_1_ = 3;
  FUN_005f52c0(param_1 + 0x21);
  local_4._0_1_ = 4;
  FUN_005f2f50(param_1 + 0x26);
  local_4._0_1_ = 5;
  FUN_005f2f00(param_1 + 0x66);
  local_4._0_1_ = 6;
  FUN_005f2f00(param_1 + 0x69);
  local_4._0_1_ = 7;
  FUN_00618c80(param_1 + 0x6c,0x20c,0x11,FUN_005f85c0);
  local_4._0_1_ = 8;
  FUN_005fb0d0(param_1 + 0x91f);
  param_1[9] = 0;
  DAT_006be3a4 = 0;
  local_4 = CONCAT31(local_4._1_3_,9);
  param_1[0x25] = 0;
  param_1[0x24] = 0;
  param_1[0x33] = 0;
  param_1[0x31] = 0;
  param_1[0x32] = 0;
  DVar1 = timeGetTime();
  param_1[0x34] = DVar1;
  param_1[0x35] = 0;
  param_1[0x30] = 0;
  param_1[0x3e] = 0;
  param_1[0x3f] = 0;
  param_1[0x40] = 0;
  param_1[0x41] = 0;
  param_1[0x36] = 0;
  param_1[0x37] = 0;
  param_1[0x38] = 0;
  param_1[0x39] = 0;
  param_1[0x3c] = 0;
  param_1[0x3d] = 0;
  param_1[0x3a] = 0;
  param_1[0x3b] = 0;
  param_1[0xc] = 0;
  FUN_00618ed0((undefined1 *)(param_1 + 0x45),&DAT_006ac380);
  param_1[0x65] = 1000;
  DAT_006be3a8 = param_1;
  param_1[0xb] = 0;
  param_1[0x43] = 0xffffffff;
  DAT_006ac294 = 0;
  (**(code **)param_1[0x91f])();
  param_1[0x42] = 0;
  ExceptionList = pvStack_c;
  return param_1;
}

