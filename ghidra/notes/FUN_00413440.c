
void __fastcall FUN_00413440(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062aa00;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_006588f8;
  local_4 = 4;
  puVar1 = (undefined4 *)FUN_005f6c30();
  if (puVar1 != (undefined4 *)0x0) {
    (**(code **)*puVar1)(1);
  }
  puVar1 = (undefined4 *)param_1[0x2e];
  if ((puVar1 != (undefined4 *)0x0) && (puVar1 != (undefined4 *)param_1[0x30])) {
    if (puVar1 != (undefined4 *)0x0) {
      (**(code **)*puVar1)(1);
    }
    param_1[0x2e] = 0;
  }
  puVar1 = (undefined4 *)param_1[0x2f];
  if ((puVar1 != (undefined4 *)0x0) && (puVar1 != (undefined4 *)param_1[0x30])) {
    if (puVar1 != (undefined4 *)0x0) {
      (**(code **)*puVar1)(1);
    }
    param_1[0x2f] = 0;
  }
  if ((undefined4 *)param_1[0x30] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x30])(1);
    param_1[0x30] = 0;
  }
  if ((undefined4 *)param_1[0x31] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x31])(1);
    param_1[0x31] = 0;
  }
  FUN_00401bf0();
  FUN_00600e20();
  local_4._0_1_ = 3;
  FUN_005f2f40(param_1 + 0x52);
  local_4._0_1_ = 2;
  FUN_005fbba0(param_1 + 0x44);
  local_4._0_1_ = 1;
  FUN_005f2ff0(param_1 + 0x34);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_005fb980(param_1);
  ExceptionList = pvStack_c;
  return;
}

