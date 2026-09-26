
undefined4 __fastcall FUN_0051e320(int param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643299;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 4) = 1;
  *(undefined4 *)(param_1 + 8) = 1;
  *(undefined4 *)(param_1 + 0xc) = 1;
  *(undefined4 *)(param_1 + 0x10) = 1;
  *(undefined4 *)(param_1 + 0x14) = 0;
  *(undefined4 *)(param_1 + 0x18) = 0;
  *(undefined4 *)(param_1 + 0x1c) = 1;
  *(undefined4 *)(param_1 + 0x20) = 0;
  *(undefined4 *)(param_1 + 0x24) = 0;
  *(undefined4 *)(param_1 + 0x28) = 0;
  *(undefined4 *)(param_1 + 0x2c) = 0;
  *(undefined4 *)(param_1 + 0x30) = 0;
  *(undefined4 *)(param_1 + 0x34) = 0;
  *(undefined4 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x3c) = 0;
  *(undefined4 *)(param_1 + 0x40) = 0;
  *(undefined4 *)(param_1 + 0x44) = 0;
  *(undefined4 *)(param_1 + 0x48) = 0;
  *(undefined4 *)(param_1 + 0x4c) = 0;
  *(undefined4 *)(param_1 + 0xa4) = 0;
  *(undefined4 *)(param_1 + 0xa8) = 0;
  *(undefined4 *)(param_1 + 0xac) = 0;
  *(undefined4 *)(param_1 + 0xb0) = 0;
  *(undefined4 *)(param_1 + 0xb4) = 0;
  *(undefined4 *)(param_1 + 0xb8) = 0;
  *(undefined4 *)(param_1 + 0xbc) = 0;
  *(undefined4 *)(param_1 + 0xc0) = 0;
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00536eb0(puVar1);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0xa4) = puVar1;
  if ((puVar1 == (undefined4 *)0x0) || (puVar1[3] == 0)) {
    *(undefined4 *)(param_1 + 4) = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_0054ed40(puVar1);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0xa8) = puVar1;
  if ((puVar1 == (undefined4 *)0x0) || (puVar1[3] == 0)) {
    *(undefined4 *)(param_1 + 4) = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 2;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_0054ede0(puVar1);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0xac) = puVar1;
  if ((puVar1 == (undefined4 *)0x0) || (puVar1[3] == 0)) {
    *(undefined4 *)(param_1 + 4) = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 3;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_0054ee40(puVar1);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0xb0) = puVar1;
  if ((puVar1 == (undefined4 *)0x0) || (puVar1[3] == 0)) {
    *(undefined4 *)(param_1 + 4) = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x14);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 4;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00568840(puVar1);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0xb4) = puVar1;
  if ((puVar1 == (undefined4 *)0x0) || (puVar1[4] == 0)) {
    *(undefined4 *)(param_1 + 4) = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x14);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 5;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00568840(puVar1);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0xb8) = puVar1;
  if ((puVar1 == (undefined4 *)0x0) || (puVar1[4] == 0)) {
    *(undefined4 *)(param_1 + 4) = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0xc);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 6;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00568b40(puVar1);
  }
  *(undefined4 **)(param_1 + 0xbc) = puVar1;
  if ((puVar1 == (undefined4 *)0x0) || (puVar1[2] == 0)) {
    *(undefined4 *)(param_1 + 4) = 0;
  }
  ExceptionList = local_c;
  return *(undefined4 *)(param_1 + 4);
}

