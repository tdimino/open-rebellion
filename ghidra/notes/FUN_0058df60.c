
int __thiscall FUN_0058df60(void *this,void *param_1)

{
  int iVar1;
  undefined4 uVar2;
  void *pvVar3;
  int iVar4;
  int iVar5;
  int **ppiVar6;
  uint local_a0;
  void *local_9c;
  int local_98;
  int *local_94;
  uint local_90;
  int aiStack_8c [2];
  int iStack_84;
  undefined4 local_7c [7];
  int local_60;
  undefined4 local_5c [7];
  int iStack_40;
  int aiStack_3c [4];
  int aiStack_2c [3];
  int iStack_20;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006500f3;
  local_c = ExceptionList;
  iVar4 = 1;
  if (*(int *)((int)this + 0x30) == 0) {
    local_98 = 0;
    ExceptionList = &local_c;
    local_9c = this;
    FUN_004ffef0(local_7c,*(undefined4 *)((int)this + 0x10),1,*(undefined4 *)((int)this + 0x14));
    local_4 = 0;
    FUN_00513120((int)local_7c);
    do {
      if (local_60 == 0) break;
      iVar1 = FUN_0052bed0((int)local_7c);
      local_90 = *(uint *)(iVar1 + 0x58) >> 6 & 1;
      if (local_90 != 0) {
        uVar2 = FUN_0052bed0((int)local_7c);
        iVar1 = FUN_004fd6b0(uVar2);
        iVar5 = 0;
        local_a0 = 0;
        this = local_9c;
        if (iVar4 != 0) {
          ppiVar6 = &local_94;
          iVar4 = 2;
          local_94 = (int *)0x0;
          pvVar3 = (void *)FUN_0052bed0((int)local_7c);
          iVar4 = FUN_004fd790(pvVar3,iVar4,(int *)ppiVar6);
          this = local_9c;
          if (iVar4 != 0) {
            if (local_94 != (int *)0x0) {
              iVar5 = (**(code **)(*local_94 + 500))();
            }
            this = local_9c;
            if ((iVar4 != 0) &&
               (iVar4 = FUN_0055e950(iVar1,iVar5,(int *)&local_a0), this = local_9c, iVar4 != 0)) {
              local_98 = local_98 + local_a0;
            }
          }
        }
      }
      FUN_005130d0((int)local_7c);
    } while (iVar4 != 0);
    iVar1 = 0;
    FUN_00527940(local_5c,*(int *)((int)this + 0x10),1,
                 *(uint *)(*(int *)((int)this + 0x10) + 0x24) >> 6 & 3);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00513120((int)local_5c);
    if (iVar4 != 0) {
      while (iStack_40 != 0) {
        iVar5 = FUN_0052bed0((int)local_5c);
        iVar5 = FUN_00520b80(iVar5);
        iVar1 = iVar1 + iVar5;
        FUN_005130d0((int)local_5c);
      }
    }
    local_94 = (int *)0x0;
    if ((iVar4 != 0) && (iVar1 < local_98)) {
      local_94 = (int *)(local_98 - iVar1);
    }
    if (((local_94 != (int *)0x0) && (*(int *)((int)this + 0x40) != 0)) &&
       (iVar1 = *(int *)((int)this + 0x24),
       iVar5 = FUN_0053e290(*(int *)((int)this + 0x20) + -1 + iVar1), iVar5 < iVar1)) {
      local_90 = 0;
      FUN_0058d080((void *)((int)this + 8),&local_90,aiStack_8c);
      local_9c = (void *)0x0;
      iVar4 = FUN_0058cf80(aiStack_8c,(int *)&local_9c);
      local_a0 = 0;
      if (((iVar4 != 0) && (iVar4 = FUN_0055e970((int)local_9c,&local_a0), iVar4 != 0)) &&
         (local_a0 != 0)) {
        iVar4 = FUN_0058d020((void *)((int)this + 8),aiStack_8c,param_1);
      }
    }
    for (local_98 = 0;
        ((iVar4 != 0 && (*(int *)((int)this + 0x30) == 0)) && (local_98 < (int)local_94));
        local_98 = local_98 + 1) {
      pvVar3 = (void *)((int)this + 8);
      local_9c = (void *)0x0;
      FUN_0058d290(aiStack_2c,pvVar3);
      local_4 = CONCAT31(local_4._1_3_,2);
      iStack_20 = FUN_0053e290(*(int *)((int)this + 0x20) + -1);
      if (iVar4 != 0) {
        iVar4 = FUN_0058cba0(pvVar3,aiStack_2c,param_1);
      }
      FUN_0058d370(aiStack_2c,&local_9c,aiStack_3c);
      if ((iVar4 != 0) && (local_9c != (void *)0x0)) {
        local_a0 = 0;
        iVar4 = FUN_0058cf80(aiStack_3c,(int *)&local_a0);
        local_90 = 0;
        if ((iVar4 != 0) &&
           ((iVar4 = FUN_0055e970(local_a0,&local_90), iVar4 != 0 && (local_90 != 0)))) {
          iVar4 = FUN_0058d020(pvVar3,aiStack_3c,param_1);
          FUN_0058d1c0(aiStack_8c,pvVar3);
          local_4._0_1_ = 3;
          iVar1 = FUN_0058cba0(pvVar3,aiStack_8c,param_1);
          if ((iVar1 == 0) || (iVar4 == 0)) {
            iVar4 = 0;
          }
          else {
            iVar4 = 1;
          }
          if (iStack_84 == 0) {
            *(undefined4 *)((int)this + 0x30) = 1;
          }
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_0058d200(aiStack_8c);
        }
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_0058d2e0(aiStack_2c);
    }
    local_4 = local_4 & 0xffffff00;
    FUN_005279c0(local_5c);
    local_4 = 0xffffffff;
    FUN_004fff70(local_7c);
  }
  ExceptionList = local_c;
  return iVar4;
}

