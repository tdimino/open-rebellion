
uint __cdecl FUN_00553b80(int param_1,uint *param_2)

{
  int iVar1;
  uint uVar2;
  uint local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006490f0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_0041c210(param_1);
  uVar2 = (uint)(iVar1 != 0);
  if (uVar2 != 0) {
    FUN_00520570(local_1c + 2);
    local_1c[0] = 1;
    local_1c[1] = 0x12;
    local_4._0_1_ = 1;
    local_4._1_3_ = 0;
    uVar2 = FUN_00504e60((uint *)(iVar1 + 0x3c),&param_1,local_1c,local_1c + 2);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if (uVar2 != 0) {
      uVar2 = (uint)(param_1 != 0);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (uVar2 != 0) {
      uVar2 = *(uint *)(param_1 + 0x24) >> 6 & 3;
      *param_2 = uVar2;
      if ((uVar2 != 0) && (uVar2 < 4)) {
        ExceptionList = local_c;
        return 1;
      }
      uVar2 = 0;
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

