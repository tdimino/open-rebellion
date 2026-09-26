
void __cdecl FUN_0055e1f0(undefined4 *param_1)

{
  undefined4 *puVar1;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649eb8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = FUN_00562ae0(local_28);
  local_4 = 0;
  FUN_00562b80(param_1,puVar1);
  local_4 = 0xffffffff;
  FUN_00562b30();
  *param_1 = 0;
  FUN_0055de00((int)param_1);
  param_1[3] = 1;
  param_1[4] = DAT_006bb6b8;
  param_1[5] = 1;
  param_1[6] = DAT_006bb5d0;
  ExceptionList = local_c;
  return;
}

