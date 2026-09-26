
int __fastcall FUN_005091f0(undefined4 param_1)

{
  void *this;
  int *piVar1;
  int iVar2;
  undefined1 *puVar3;
  undefined1 local_30 [4];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641920;
  pvStack_c = ExceptionList;
  iVar2 = 0;
  ExceptionList = &pvStack_c;
  FUN_00504cc0(local_2c,param_1,1,2);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    puVar3 = local_30;
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4._0_1_ = 1;
    this = (void *)FUN_0052bed0((int)local_2c);
    piVar1 = FUN_00402d80(this,puVar3);
    iVar2 = iVar2 + (uint)(*piVar1 == 0x10000006);
    FUN_00619730();
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_00504d40(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

