
undefined4 * __fastcall FUN_00581340(undefined4 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064eb97;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0051fa20(param_1);
  uVar2 = param_1[7];
  local_4 = 0;
  param_1[0x11] = 0;
  *param_1 = &PTR_FUN_00669e70;
  if (uVar2 != 0) {
    puVar1 = (undefined4 *)FUN_00618b70(0x10);
    if (puVar1 == (undefined4 *)0x0) {
      FUN_00619730();
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = FUN_0054ede0(puVar1);
    }
    param_1[0x11] = puVar1;
    uVar2 = (uint)(puVar1 != (undefined4 *)0x0);
  }
  param_1[7] = uVar2;
  ExceptionList = local_c;
  return param_1;
}

