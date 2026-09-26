
undefined4 * __fastcall FUN_004ebae0(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e9c3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ecc00(param_1);
  local_4 = 0;
  FUN_005f4950(param_1 + 9,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005f4950(param_1 + 10,0);
  *param_1 = &PTR_FUN_0065ca10;
  param_1[8] = 0;
  param_1[0xe] = 0xffffffff;
  param_1[0xf] = 0;
  *(undefined2 *)(param_1 + 0xb) = 0;
  *(undefined2 *)((int)param_1 + 0x2e) = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  puVar2 = param_1 + 0x10;
  for (iVar1 = 0x16; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  param_1[0x26] = 0;
  param_1[7] = 0;
  ExceptionList = local_c;
  return param_1;
}

