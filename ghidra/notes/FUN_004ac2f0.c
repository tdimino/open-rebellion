
void __fastcall FUN_004ac2f0(undefined4 *param_1)

{
  undefined *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00638564;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065bf80;
  local_4 = 2;
  if ((undefined4 *)param_1[0x4e] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4e])(1);
  }
  if ((undefined4 *)param_1[0x4f] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4f])(1);
  }
  puVar1 = (undefined *)param_1[0x49];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  local_4._0_1_ = 1;
  FUN_004f4380(param_1 + 0x47);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004acb90(param_1 + 0x45);
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = pvStack_c;
  return;
}

