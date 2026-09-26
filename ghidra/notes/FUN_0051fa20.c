
undefined4 * __fastcall FUN_0051fa20(undefined4 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064335d;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f5c70(param_1);
  local_4 = 0;
  FUN_004f4340(param_1 + 0xb);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0xd);
  local_4 = CONCAT31(local_4._1_3_,2);
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x10] = 0;
  *param_1 = &PTR_FUN_0065ed28;
  uVar2 = 0;
  if (param_1[7] != 0) {
    puVar1 = (undefined4 *)FUN_00618b70(0x10);
    if (puVar1 == (undefined4 *)0x0) {
      FUN_00619730();
    }
    local_4 = CONCAT31(local_4._1_3_,3);
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = FUN_0054ede0(puVar1);
    }
    uVar2 = (uint)(puVar1 != (undefined4 *)0x0);
    param_1[0x10] = puVar1;
  }
  param_1[7] = uVar2;
  ExceptionList = local_c;
  return param_1;
}

