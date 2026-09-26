
/* WARNING: Type propagation algorithm not settling */

undefined4 __fastcall FUN_005637c0(int param_1)

{
  int iVar1;
  int local_84 [5];
  undefined1 local_70 [28];
  undefined4 local_54 [18];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a690;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_70,param_1 + 0x20);
  local_84[0] = 0;
  local_84[1] = 1;
  local_84[2] = -1;
  local_84[3] = 1;
  local_84[4] = 0x12;
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  iVar1 = FUN_00506000((uint *)(param_1 + 0x3c),local_84,(uint *)(local_84 + 3),local_84 + 1);
  local_4._0_1_ = 1;
  FUN_00619730();
  if ((iVar1 != 0) && (local_84[2] == -1)) {
    FUN_0058d720(local_54);
    local_4._0_1_ = 3;
    FUN_0058d800(local_54,local_84[0],param_1 + 0x40,local_70);
    local_4._0_1_ = 1;
    FUN_0058d760(local_54);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004fd4d0((undefined4 *)local_70);
  ExceptionList = local_c;
  return 0;
}

