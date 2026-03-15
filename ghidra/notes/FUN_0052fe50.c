
uint __fastcall FUN_0052fe50(undefined4 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  uint *puVar3;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006453e7;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = param_1;
  puVar1 = (undefined4 *)FUN_00618b70(0x40);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  local_10 = puVar1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_005777c0(puVar1);
  }
  local_4 = 0xffffffff;
  uVar2 = (uint)(puVar1 != (undefined4 *)0x0);
  if (uVar2 != 0) {
    uVar2 = puVar1[6];
    if (uVar2 == 0) {
      if (puVar1 != (undefined4 *)0x0) {
        (**(code **)*puVar1)(1);
      }
    }
    else {
      local_4 = 0xffffffff;
      puVar3 = FUN_004025b0(param_1,(uint *)&local_10);
      local_4 = 1;
      FUN_004f26d0(puVar1 + 0xf,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_004fd390(puVar1);
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

