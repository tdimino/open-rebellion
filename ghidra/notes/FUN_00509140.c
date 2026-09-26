
int __thiscall FUN_00509140(void *this,undefined4 param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *pvVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641900;
  pvStack_c = ExceptionList;
  iVar4 = 0;
  ExceptionList = &pvStack_c;
  FUN_004ffef0(local_4c,this,param_2,param_1);
  local_4 = 0;
  FUN_00513120((int)local_4c);
  while (local_30 != 0) {
    uVar5 = param_2;
    uVar6 = param_1;
    uVar1 = FUN_0052bed0((int)local_4c);
    pvVar2 = FUN_00500450(local_2c,uVar1,uVar5,uVar6);
    local_4._0_1_ = 1;
    iVar3 = FUN_00513180((int)pvVar2);
    iVar4 = iVar4 + iVar3;
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005004d0(local_2c);
    FUN_005130d0((int)local_4c);
  }
  local_4 = 0xffffffff;
  FUN_004fff70(local_4c);
  ExceptionList = pvStack_c;
  return iVar4;
}

