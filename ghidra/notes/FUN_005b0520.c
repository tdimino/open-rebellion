
void __fastcall FUN_005b0520(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006530d8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066c140;
  param_1[0x35] = &PTR_LAB_0066c138;
  param_1[0x3c] = &PTR_LAB_0066c130;
  param_1[0x3f] = &PTR_LAB_0066c12c;
  param_1[0x41] = &PTR_LAB_0066c120;
  param_1[0x43] = &PTR_LAB_0066c118;
  param_1[0x106] = &PTR_LAB_0066c110;
  local_4 = 0;
  FUN_005b83b0((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0x106)));
  local_4 = 0xffffffff;
  FUN_005b12d0(param_1);
  ExceptionList = local_c;
  return;
}

