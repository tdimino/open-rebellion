
void __fastcall FUN_0041bee0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_0062b678;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658a58;
  param_1[3] = &PTR_LAB_00658a68;
  local_4 = 1;
  thunk_FUN_005f5b20(param_1);
  local_4 = local_4 & 0xffffff00;
  FUN_005f5bc0((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 3)));
  local_4 = 0xffffffff;
  FUN_005f59e0(param_1);
  ExceptionList = pvStack_c;
  return;
}

