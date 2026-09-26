
/* WARNING: Type propagation algorithm not settling */

undefined4 __fastcall FUN_00563390(int param_1)

{
  int iVar1;
  int local_6c [5];
  undefined1 local_58 [28];
  undefined4 local_3c [12];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a5e0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_58,param_1 + 0x20);
  local_6c[0] = 0;
  local_6c[1] = 1;
  local_6c[2] = -1;
  local_6c[3] = 1;
  local_6c[4] = 0x12;
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  iVar1 = FUN_00506000((uint *)(param_1 + 0x3c),local_6c,(uint *)(local_6c + 3),local_6c + 1);
  local_4._0_1_ = 1;
  FUN_00619730();
  if ((iVar1 != 0) && (local_6c[2] == -1)) {
    FUN_0058c0d0(local_3c);
    local_4._0_1_ = 3;
    FUN_0058c1e0(local_3c,local_6c[0],param_1 + 0x40,local_58);
    local_4._0_1_ = 1;
    FUN_0058c110(local_3c);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004fd4d0((undefined4 *)local_58);
  ExceptionList = local_c;
  return 0;
}

