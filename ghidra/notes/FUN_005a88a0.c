
void __fastcall FUN_005a88a0(undefined4 *param_1)

{
  int iVar1;
  int *piVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00652d9d;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_LAB_0066c2d0;
  local_4 = 0x12;
  FUN_005a9010((int)param_1);
  piVar2 = param_1 + 2;
  iVar1 = 2;
  do {
    if ((undefined4 *)*piVar2 != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)*piVar2)(1);
      *piVar2 = 0;
    }
    piVar2 = piVar2 + 1;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  local_4._0_1_ = 0x11;
  FUN_005bdf10(param_1 + 0x397);
  local_4._0_1_ = 0x10;
  FUN_005bdaa0(param_1 + 0x394);
  local_4._0_1_ = 0xf;
  FUN_005ca3e0(param_1 + 0x291);
  local_4._0_1_ = 0xe;
  FUN_005c5b60(param_1 + 0x277);
  local_4._0_1_ = 0xd;
  FUN_005c5ff0(param_1 + 0x274);
  local_4._0_1_ = 0xc;
  FUN_005c5ff0(param_1 + 0x271);
  local_4._0_1_ = 0xb;
  FUN_005c5ff0(param_1 + 0x26e);
  local_4._0_1_ = 10;
  FUN_005c5ff0(param_1 + 0x26b);
  local_4._0_1_ = 9;
  FUN_005c5ff0(param_1 + 0x268);
  local_4._0_1_ = 8;
  FUN_005c5ff0(param_1 + 0x265);
  local_4._0_1_ = 7;
  FUN_005c5ff0(param_1 + 0x262);
  local_4._0_1_ = 6;
  FUN_00618d20(param_1 + 0x24e,0xc,2,FUN_005ce5b0);
  local_4._0_1_ = 5;
  FUN_00618d20(param_1 + 0x248,0xc,2,FUN_005ce5b0);
  local_4._0_1_ = 4;
  FUN_00618d20(param_1 + 0x242,0xc,2,FUN_005ce5b0);
  local_4._0_1_ = 3;
  FUN_005cda30(param_1 + 0x240);
  local_4._0_1_ = 2;
  FUN_005cda30(param_1 + 0x23e);
  local_4._0_1_ = 1;
  FUN_005f2ff0(param_1 + 0x237);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00618d20(param_1 + 0x184,0xc,0x26,FUN_005f2ff0);
  local_4 = 0xffffffff;
  FUN_00618d20(param_1 + 4,0xc,0x80,FUN_005f2ff0);
  ExceptionList = pvStack_c;
  return;
}

