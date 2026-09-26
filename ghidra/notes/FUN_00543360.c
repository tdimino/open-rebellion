
undefined4 FUN_00543360(int param_1,undefined4 *param_2)

{
  bool bVar1;
  undefined4 *this;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647238;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece80(param_2);
  FUN_004ece30(&param_2);
  local_4 = 0;
  uVar2 = *(uint *)(param_1 + 0x74) >> 0x18;
  if ((uVar2 < 0x90) || (0x97 < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    FUN_004f26d0(this,(undefined4 *)(param_1 + 0x70));
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

