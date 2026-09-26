
undefined4 __cdecl FUN_0055e150(undefined4 *param_1)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649e98;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = FUN_00562ae0(local_28);
  local_4 = 0;
  FUN_00562b80(param_1,puVar2);
  local_4 = 0xffffffff;
  FUN_00562b30();
  *param_1 = 1;
  FUN_0055de00((int)param_1);
  param_1[3] = 1;
  uVar1 = DAT_006bb638;
  param_1[5] = 1;
  param_1[4] = uVar1;
  param_1[6] = DAT_006bb63c;
  ExceptionList = local_c;
  return 1;
}

