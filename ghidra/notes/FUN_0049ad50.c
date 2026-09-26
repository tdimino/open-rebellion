
void __fastcall FUN_0049ad50(undefined4 *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  undefined1 local_14 [4];
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00636d89;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065bc08;
  local_4 = 0xd;
  local_10 = param_1;
  for (pvVar1 = (void *)thunk_FUN_005f5060((int)(param_1 + 0x1e)); pvVar1 != (void *)0x0;
      pvVar1 = *(void **)((int)pvVar1 + 0x10)) {
    puVar2 = FUN_00403040(pvVar1,local_14);
    local_4._0_1_ = 0xe;
    FUN_0041dc80(param_1[8],puVar2);
    local_4 = CONCAT31(local_4._1_3_,0xd);
    FUN_00619730();
  }
  for (pvVar1 = (void *)thunk_FUN_005f5060((int)(param_1 + 0x20)); pvVar1 != (void *)0x0;
      pvVar1 = *(void **)((int)pvVar1 + 0x10)) {
    puVar2 = FUN_00403040(pvVar1,local_14);
    local_4._0_1_ = 0xf;
    FUN_0041dc80(param_1[8],puVar2);
    local_4 = CONCAT31(local_4._1_3_,0xd);
    FUN_00619730();
  }
  local_4._1_3_ = (uint3)((uint)local_4 >> 8);
  local_4._0_1_ = 0xc;
  FUN_005f2ff0(param_1 + 0x2e);
  local_4._0_1_ = 0xb;
  FUN_005f2ff0(param_1 + 0x2b);
  local_4._0_1_ = 10;
  FUN_005f2ff0(param_1 + 0x28);
  local_4._0_1_ = 9;
  FUN_005f2ff0(param_1 + 0x25);
  local_4._0_1_ = 8;
  FUN_005f2ff0(param_1 + 0x22);
  local_4._0_1_ = 7;
  FUN_004f55c0(param_1 + 0x20);
  local_4._0_1_ = 6;
  FUN_004f55c0(param_1 + 0x1e);
  local_4._0_1_ = 5;
  FUN_004f55c0(param_1 + 0x1c);
  local_4._0_1_ = 4;
  FUN_004f55c0(param_1 + 0x1a);
  local_4._0_1_ = 3;
  FUN_004f55c0(param_1 + 0x18);
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004c4e30(param_1);
  ExceptionList = local_c;
  return;
}

