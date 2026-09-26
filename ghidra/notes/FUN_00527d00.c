
uint __thiscall FUN_00527d00(void *this,int param_1,void *param_2)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006444a0;
  pvStack_c = ExceptionList;
  local_34 = 1;
  local_30 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  uVar1 = FUN_004f9860(this,param_1,param_2);
  if (uVar1 != 0) {
    if (*(int *)((int)param_2 + 4) == -1) {
      iVar2 = FUN_0052bed0((int)this);
      uVar1 = (uint)(iVar2 != 0);
      iVar4 = 0;
      FUN_0052cda0(local_2c,iVar2);
      local_4 = 1;
      FUN_00513120((int)local_2c);
      iVar2 = FUN_0052bed0((int)local_2c);
      while (iVar2 != 0) {
        piVar3 = (int *)FUN_0052bed0((int)local_2c);
        iVar2 = FUN_0052bde0(this,piVar3);
        if ((iVar2 == 0) ||
           (iVar2 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar2 + 0x60) & 1) != 0)) {
          iVar2 = 0;
        }
        else {
          iVar2 = 1;
        }
        iVar4 = iVar4 + iVar2;
        FUN_005130d0((int)local_2c);
        iVar2 = FUN_0052bed0((int)local_2c);
      }
      if (iVar4 == 0) {
        local_34 = 0xa0;
        local_30 = 3;
        local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
        local_4._0_1_ = 2;
        FUN_00520580(param_2,&local_34);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
      }
      local_4 = 0xffffffff;
      FUN_0052cec0(local_2c);
    }
    if (uVar1 != 0) {
      ExceptionList = pvStack_c;
      return uVar1;
    }
  }
  local_34 = 1;
  local_30 = 1;
  local_4 = 3;
  FUN_00520580(param_2,&local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return 0;
}

