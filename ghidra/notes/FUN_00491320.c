
void __fastcall FUN_00491320(undefined4 *param_1)

{
  void *this;
  undefined4 *puVar1;
  undefined1 local_14 [4];
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00635ccf;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065b6c8;
  local_4 = 4;
  local_10 = param_1;
  for (this = (void *)thunk_FUN_005f5060((int)(param_1 + 0x22)); this != (void *)0x0;
      this = *(void **)((int)this + 0x10)) {
    puVar1 = FUN_00403040(this,local_14);
    local_4._0_1_ = 5;
    FUN_0041dc80(param_1[8],puVar1);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00619730();
  }
  local_4._1_3_ = (uint3)((uint)local_4 >> 8);
  local_4._0_1_ = 3;
  FUN_004f55c0(param_1 + 0x22);
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(param_1 + 0x1a);
  local_4 = 0xffffffff;
  FUN_004c4bd0(param_1);
  ExceptionList = local_c;
  return;
}

