
int __thiscall FUN_0050b310(void *this,void *param_1)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 local_ac [7];
  int local_90;
  undefined4 local_8c [7];
  int local_70;
  undefined4 local_6c [8];
  undefined4 local_4c [8];
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641b4e;
  pvStack_c = ExceptionList;
  iVar3 = 100;
  ExceptionList = &pvStack_c;
  FUN_00527550(local_2c,this,1);
  iVar4 = 0;
  local_4 = 0;
  iVar1 = FUN_00513180((int)local_2c);
  if (((*(byte *)((int)this + 0x88) & 0x20) != 0) && (iVar1 == 0)) {
    iVar1 = 0;
    FUN_004ffe70(local_8c,this,1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00513120((int)local_8c);
    while (local_70 != 0) {
      uVar5 = 1;
      uVar2 = FUN_0052bed0((int)local_8c);
      FUN_00502db0(local_ac,uVar2,uVar5);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00513120((int)local_ac);
      while (local_90 != 0) {
        uVar5 = 1;
        iVar4 = iVar4 + 1;
        uVar2 = FUN_0052bed0((int)local_ac);
        FUN_005039d0(local_6c,uVar2,uVar5);
        local_4._0_1_ = 3;
        iVar3 = FUN_00513180((int)local_6c);
        iVar1 = iVar1 + iVar3;
        local_4 = CONCAT31(local_4._1_3_,2);
        FUN_00503ad0(local_6c);
        FUN_005130d0((int)local_ac);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00502eb0(local_ac);
      FUN_005130d0((int)local_8c);
    }
    FUN_005039d0(local_4c,this,1);
    local_4._0_1_ = 4;
    iVar3 = FUN_00513180((int)local_4c);
    iVar3 = FUN_0055a020(iVar4,iVar1 + iVar3);
    local_4._0_1_ = 1;
    FUN_00503ad0(local_4c);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_004fff70(local_8c);
  }
  iVar4 = FUN_00509fc0(this,iVar3,param_1);
  local_4 = 0xffffffff;
  FUN_00527650(local_2c);
  ExceptionList = pvStack_c;
  return iVar4;
}

