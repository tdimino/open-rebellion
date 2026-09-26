
void __fastcall FUN_005b12d0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00653317;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066c560;
  param_1[0x35] = &PTR_LAB_0066c558;
  param_1[0x3c] = &PTR_LAB_0066c550;
  param_1[0x3f] = &PTR_LAB_0066c54c;
  param_1[0x41] = &PTR_LAB_0066c540;
  param_1[0x43] = &PTR_LAB_0066c53c;
  local_4 = 9;
  FUN_005c5ff0(param_1 + 0xd8);
  local_4._0_1_ = 8;
  FUN_00618d20(param_1 + 0x5f,0x74,4,FUN_005b0ee0);
  local_4._0_1_ = 7;
  FUN_005f2ff0(param_1 + 0x51);
  local_4._0_1_ = 6;
  FUN_005bdcc0(param_1 + 0x48);
  local_4._0_1_ = 5;
  FUN_005bdd00(param_1 + 0x46);
  local_4 = (uint)local_4._1_3_ << 8;
  *(undefined ***)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0x43)) =
       &PTR_LAB_0066bde8;
  *(undefined ***)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0x41)) =
       &PTR_LAB_0066be18;
  *(undefined ***)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0x3f)) =
       &PTR_LAB_0066be24;
  *(undefined ***)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0x3c)) =
       &PTR_LAB_0066be28;
  FUN_005a6280((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0x35)));
  local_4 = 0xffffffff;
  FUN_005cd2c0(param_1);
  ExceptionList = pvStack_c;
  return;
}

