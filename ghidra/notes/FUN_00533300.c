
uint __fastcall FUN_00533300(int param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 *local_1c [2];
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006457e5;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00520570(local_1c);
  local_4 = 0;
  FUN_00520580((void *)(param_1 + 0x58),puVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar1 = (undefined4 *)FUN_00520570(local_1c);
  local_4 = 1;
  FUN_00520580((void *)(param_1 + 0x60),puVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar1 = (undefined4 *)FUN_00520570(local_1c);
  local_4 = 2;
  FUN_00520580((void *)(param_1 + 0x68),puVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  *(undefined4 *)(param_1 + 0x70) = 0;
  *(undefined4 *)(param_1 + 0x74) = 0;
  *(undefined4 *)(param_1 + 0x78) = 0;
  *(undefined4 *)(param_1 + 0x7c) = 0;
  *(undefined4 *)(param_1 + 0x80) = 0;
  *(undefined4 *)(param_1 + 0x84) = 0;
  *(undefined4 *)(param_1 + 0x88) = 0;
  *(undefined4 *)(param_1 + 0x8c) = 0;
  *(undefined4 *)(param_1 + 0x90) = 0;
  *(undefined4 *)(param_1 + 0x94) = 0;
  *(undefined4 *)(param_1 + 0x98) = 0;
  *(undefined4 *)(param_1 + 0x9c) = 0;
  *(undefined4 *)(param_1 + 0xa0) = 0;
  *(undefined4 *)(param_1 + 0xa4) = 0;
  *(undefined4 *)(param_1 + 0xa8) = 0;
  *(undefined4 *)(param_1 + 0xac) = 0;
  *(undefined4 *)(param_1 + 0xb0) = 0;
  *(undefined4 *)(param_1 + 0xb4) = 0;
  *(undefined4 *)(param_1 + 0xb8) = 0;
  *(undefined4 *)(param_1 + 0xbc) = 0;
  *(undefined4 *)(param_1 + 0xc0) = 100;
  *(undefined4 *)(param_1 + 0xc4) = 1;
  *(undefined4 *)(param_1 + 200) = 0;
  puVar1 = FUN_005401b0(local_1c);
  local_4 = 3;
  FUN_005401e0((void *)(param_1 + 0xcc),puVar1);
  local_4 = 0xffffffff;
  thunk_FUN_00619730();
  puVar1 = FUN_005401b0(local_14);
  local_4 = 4;
  FUN_005401e0((void *)(param_1 + 0xd4),puVar1);
  local_4 = 0xffffffff;
  thunk_FUN_00619730();
  local_1c[0] = (undefined4 *)FUN_00618b70(0x14);
  if (local_1c[0] == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 5;
  if (local_1c[0] == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00583950(local_1c[0]);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0x88) = puVar1;
  uVar2 = 0;
  if ((puVar1 != (undefined4 *)0x0) && (uVar2 = 0, puVar1[4] != 0)) {
    puVar1 = (undefined4 *)FUN_00618b70(0x14);
    if (puVar1 == (undefined4 *)0x0) {
      FUN_00619730();
    }
    local_4 = 6;
    local_1c[0] = puVar1;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = FUN_00583950(puVar1);
    }
    local_4 = 0xffffffff;
    *(undefined4 **)(param_1 + 0x8c) = puVar1;
    uVar2 = 0;
    if ((puVar1 != (undefined4 *)0x0) && (uVar2 = 0, puVar1[4] != 0)) {
      puVar1 = (undefined4 *)FUN_00618b70(0x10);
      if (puVar1 == (undefined4 *)0x0) {
        FUN_00619730();
      }
      local_4 = 7;
      if (puVar1 == (undefined4 *)0x0) {
        puVar1 = (undefined4 *)0x0;
      }
      else {
        local_1c[0] = puVar1;
        puVar1 = FUN_004f3f70(puVar1);
      }
      *(undefined4 **)(param_1 + 200) = puVar1;
      uVar2 = 0;
      if (puVar1 != (undefined4 *)0x0) {
        uVar2 = puVar1[3];
      }
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

