
void __fastcall FUN_00513430(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00642467;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065e938;
  param_1[0xc] = &PTR_FUN_0065e930;
  local_4 = 0xf;
  if ((param_1[9] & 0x30) == 0) {
    iVar1 = FUN_00539fd0(param_1,1);
    if (iVar1 != 0) {
      puVar2 = (undefined4 *)FUN_00539fd0(param_1,1);
      if (puVar2 != (undefined4 *)0x0) {
        (**(code **)*puVar2)(1);
      }
    }
    iVar1 = FUN_00539fd0(param_1,2);
    if (iVar1 != 0) {
      puVar2 = (undefined4 *)FUN_00539fd0(param_1,2);
      if (puVar2 != (undefined4 *)0x0) {
        (**(code **)*puVar2)(1);
      }
    }
    thunk_FUN_005674c0();
    thunk_FUN_0058b400();
    FUN_005197c0((int)param_1);
    FUN_0053f450();
    FUN_0053f560();
    DAT_006b2bb0 = 0;
    FUN_0051c010(1);
    param_1[0x2c] = 0;
    param_1[0x2d] = 0;
    param_1[0x2e] = 0;
    param_1[0x2f] = 0;
    param_1[0x30] = 0;
    param_1[0x31] = 0;
    param_1[0x32] = 0;
    param_1[0x33] = 0;
    param_1[0x34] = 0;
    param_1[0x35] = 0;
    param_1[0x36] = 0;
    param_1[0x37] = 0;
  }
  local_4._0_1_ = 0xe;
  FUN_004fd4d0(param_1 + 0x65);
  local_4._0_1_ = 0xd;
  FUN_004fd4d0(param_1 + 0x5e);
  local_4._0_1_ = 0xc;
  FUN_004fd4d0(param_1 + 0x57);
  local_4._0_1_ = 0xb;
  FUN_00619730();
  local_4._0_1_ = 10;
  FUN_00619730();
  local_4._0_1_ = 9;
  FUN_004f3fb0(param_1 + 0x50);
  local_4._0_1_ = 8;
  FUN_004f3fb0(param_1 + 0x4c);
  local_4._0_1_ = 7;
  FUN_004f3fb0(param_1 + 0x48);
  local_4._0_1_ = 6;
  FUN_004f3fb0(param_1 + 0x44);
  local_4._0_1_ = 5;
  FUN_004f3fb0(param_1 + 0x40);
  local_4._0_1_ = 4;
  FUN_004f3fb0(param_1 + 0x3c);
  local_4._0_1_ = 3;
  FUN_004f3fb0(param_1 + 0x38);
  local_4._0_1_ = 2;
  FUN_004fd4d0(param_1 + 0x24);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0051c780(param_1 + 0x16);
  local_4 = 0xffffffff;
  FUN_004f5f60(param_1);
  ExceptionList = local_c;
  return;
}

