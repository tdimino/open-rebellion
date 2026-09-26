
undefined4 * __fastcall FUN_004eceb0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ec72;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005336b0(param_1);
  *(undefined2 *)(param_1 + 0x26) = 4;
  *(undefined2 *)(param_1 + 0x1f) = 0;
  *(undefined2 *)((int)param_1 + 0x7e) = 0;
  *(undefined2 *)(param_1 + 0x20) = 0;
  *(undefined2 *)((int)param_1 + 0x82) = 0;
  *(undefined2 *)(param_1 + 0x21) = 0;
  *(undefined2 *)((int)param_1 + 0x86) = 0;
  *(undefined2 *)(param_1 + 0x22) = 0;
  *(undefined2 *)((int)param_1 + 0x8a) = 0;
  *(undefined2 *)(param_1 + 0x23) = 0;
  *(undefined2 *)((int)param_1 + 0x8e) = 0;
  *(undefined2 *)(param_1 + 0x24) = 0;
  *(undefined2 *)((int)param_1 + 0x92) = 0;
  *(undefined2 *)(param_1 + 0x25) = 0;
  *(undefined2 *)((int)param_1 + 0x96) = 0;
  local_4 = 0;
  *(undefined2 *)((int)param_1 + 0x9a) = (undefined2)DAT_006b9050;
  FUN_004ece30(param_1 + 0x27);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0x28);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0x29);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004ece30(param_1 + 0x2a);
  param_1[0x2b] = 0;
  *param_1 = &PTR_FUN_0065ca70;
  param_1[0xc] = &PTR_LAB_0065ca6c;
  ExceptionList = local_c;
  return param_1;
}

