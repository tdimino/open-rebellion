
void __fastcall FUN_005c7a90(undefined4 *param_1)

{
  int *piVar1;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00653ddd;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066cc48;
  local_4 = 9;
  if ((undefined4 *)param_1[0xb] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0xb])(1);
  }
  param_1[0xb] = 0;
  if ((undefined4 *)param_1[0xd] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0xd])(1);
  }
  piVar1 = param_1 + 0x24;
  param_1[0xd] = 0;
  iVar2 = FUN_005f5060((int)piVar1);
  if (iVar2 != 0) {
    (**(code **)(*piVar1 + 4))();
  }
  local_4._0_1_ = 8;
  param_1[0x2a] = &PTR_FUN_0066cc38;
  FUN_005f4f00(param_1 + 0x2a);
  local_4._0_1_ = 7;
  param_1[0x28] = &PTR_FUN_0066cc38;
  FUN_005f4f00(param_1 + 0x28);
  local_4._0_1_ = 6;
  param_1[0x26] = &PTR_FUN_0066cc38;
  FUN_005f4f00(param_1 + 0x26);
  local_4._0_1_ = 5;
  *piVar1 = (int)&PTR_FUN_0066cc38;
  FUN_005f4f00(piVar1);
  local_4._0_1_ = 4;
  param_1[0x22] = &PTR_FUN_0066cc28;
  FUN_005f4f00(param_1 + 0x22);
  local_4._0_1_ = 3;
  FUN_005ca3e0(param_1 + 0x1c);
  local_4._0_1_ = 2;
  FUN_005c5ff0(param_1 + 0x19);
  local_4._0_1_ = 1;
  FUN_005c5ff0(param_1 + 0x16);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005bdcc0(param_1 + 3);
  local_4 = 0xffffffff;
  FUN_005bdd00(param_1 + 1);
  ExceptionList = pvStack_c;
  return;
}

