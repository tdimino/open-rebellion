
undefined4 __fastcall FUN_00579e30(int param_1)

{
  uint uVar1;
  int iVar2;
  void *this;
  uint *puVar3;
  int *local_40;
  uint local_3c [12];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064d4e8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004fd450(local_3c + 5,param_1 + 0x20);
  local_40 = (int *)0x0;
  local_3c[3] = 1;
  local_3c[4] = 0xffffffff;
  local_3c[1] = 1;
  local_3c[2] = 0x12;
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  iVar2 = FUN_00504e60((uint *)(param_1 + 0x3c),&local_40,local_3c + 1,local_3c + 3);
  local_4._0_1_ = 1;
  FUN_00619730();
  if ((iVar2 != 0) && (local_40 != (int *)0x0)) {
    this = (void *)FUN_00506e20();
    FUN_004ece30(local_3c);
    local_4._0_1_ = 3;
    FUN_004f6b70(local_40,local_3c);
    if (this != (void *)0x0) {
      puVar3 = FUN_004025b0(this,local_3c + 1);
      uVar1 = *puVar3;
      FUN_00619730();
      if (uVar1 != local_3c[0]) {
        puVar3 = FUN_004025b0(this,local_3c + 1);
        local_4._0_1_ = 4;
        (**(code **)(*local_40 + 0xa8))(puVar3,local_3c + 5);
        local_4._0_1_ = 3;
        FUN_00619730();
      }
    }
    local_4._0_1_ = 1;
    FUN_00619730();
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_3c + 5);
  ExceptionList = pvStack_c;
  return 0;
}

