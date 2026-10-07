
undefined4 __cdecl FUN_004fad70(undefined4 param_1,void *param_2)

{
  undefined4 *puVar1;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ffd8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  puVar1 = FUN_005205a0(local_14);
  local_4 = 0;
  FUN_00520600(param_2,puVar1);
  local_4 = 0xffffffff;
  FUN_005205e0(local_14);
  ExceptionList = pvStack_c;
  return 1;
}

