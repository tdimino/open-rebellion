
void __fastcall FUN_00599ad0(undefined4 *param_1)

{
  int iVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0065174a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066bae8;
  iVar1 = param_1[0x1a4];
  local_4 = 3;
  param_1[0x1a4] = iVar1 + 1;
  if (iVar1 + 1 < 0) {
    param_1[0x1a4] = 0;
  }
  if ((undefined4 *)param_1[2] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[2])(1);
  }
  param_1[2] = 0;
  FUN_00599bc0((int)param_1);
  FUN_005ba920();
  if (DAT_006bc470 != (undefined4 *)0x0) {
    (**(code **)*DAT_006bc470)(1);
    DAT_006bc470 = (undefined4 *)0x0;
  }
  local_4._0_1_ = 2;
  FUN_005fbba0(param_1 + 0x1a6);
  local_4._0_1_ = 1;
  param_1[0x1a2] = &PTR_FUN_0066baf0;
  FUN_005f4f00(param_1 + 0x1a2);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00618d20(param_1 + 0x28,0xc,0x26,FUN_005f2ff0);
  local_4 = 0xffffffff;
  FUN_00618d20(param_1 + 4,0xc,0xc,FUN_005f2ff0);
  ExceptionList = pvStack_c;
  return;
}

