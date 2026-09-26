
/* WARNING: Type propagation algorithm not settling */

uint __thiscall FUN_0057f540(void *this,void *param_1)

{
  void *this_00;
  bool bVar1;
  int iVar2;
  uint *puVar3;
  uint uVar4;
  int iVar5;
  undefined3 extraout_var;
  uint uVar6;
  int *local_64;
  uint uStack_60;
  uint uStack_5c;
  undefined4 uStack_58;
  uint local_54;
  undefined4 local_50;
  undefined4 auStack_4c [7];
  undefined4 auStack_30 [9];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_0064e6c0;
  pvStack_c = ExceptionList;
  uVar6 = 1;
  local_64 = (int *)0x0;
  local_54 = 1;
  local_50 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_54);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (*(int *)((int)param_1 + 4) == -1) {
    local_50 = 0x12;
    local_54 = 1;
    local_4 = 1;
    uVar6 = FUN_005057f0((uint *)((int)this + 0x3c),&local_64,&local_54,param_1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if (uVar6 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      uVar6 = (**(code **)(*local_64 + 0x1c0))((int *)((int)this + 0x40),(int)this + 0x50,param_1);
      if ((uVar6 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
        FUN_00551060(auStack_30);
        local_4 = 2;
        if (*(int *)((int)param_1 + 4) == -1) {
          uStack_5c = 3;
          iVar2 = FUN_00551190(auStack_30,(int *)((int)this + 0x40),(uint)local_64[9] >> 6 & 3,
                               &uStack_5c);
          uVar6 = 0;
          if (iVar2 != 0) {
            puVar3 = FUN_004c27b0(local_64,&local_54);
            local_4._0_1_ = 3;
            uVar4 = FUN_005513a0(auStack_30,puVar3);
            local_4 = CONCAT31(local_4._1_3_,2);
            FUN_00619730();
            uVar6 = 0;
            if (uVar4 != 0) {
              uVar6 = FUN_00552210(auStack_30,local_64,param_1);
            }
          }
        }
        FUN_0042dbe0(&uStack_60);
        local_4 = CONCAT31(local_4._1_3_,4);
        param_1 = (void *)0x0;
        for (iVar2 = thunk_FUN_005f5060((int)this + 0x50);
            ((uVar6 != 0 && (*(int *)((int)this_00 + 4) == -1)) && (iVar2 != 0));
            iVar2 = *(int *)(iVar2 + 0x10)) {
          FUN_0054fa40(auStack_4c);
          local_4._0_1_ = 5;
          if (uVar6 != 0) {
            if (*(int *)((int)this_00 + 4) == -1) {
              uVar6 = FUN_00551dc0(auStack_30,(uint *)(iVar2 + 0x1c),(uint *)(iVar2 + 0x18),this_00,
                                   auStack_4c);
            }
            if ((uVar6 != 0) && (*(int *)((int)this_00 + 4) == -1)) {
              if (*(uint *)(iVar2 + 0x1c) != uStack_60) {
                FUN_004f26d0(&uStack_60,(undefined4 *)(iVar2 + 0x1c));
                uStack_5c = 0x10;
                uStack_58 = 0x40;
                if ((uStack_60 >> 0x18 < 0x10) || (0x3f < uStack_60 >> 0x18)) {
                  bVar1 = false;
                }
                else {
                  bVar1 = true;
                }
                FUN_00619730();
                if (bVar1) {
                  iVar5 = FUN_0051cab0(&uStack_60);
                  uVar6 = (uint)(iVar5 != 0);
                  if (uVar6 != 0) {
                    local_54 = FUN_0053b870(iVar5);
                  }
                }
              }
              if (uVar6 != 0) {
                param_1 = (void *)((int)param_1 + local_54);
              }
            }
          }
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_0054fa90(auStack_4c);
        }
        if ((uVar6 != 0) && (*(int *)((int)this_00 + 4) == -1)) {
          bVar1 = FUN_00518750((int)local_64,(int *)&local_54);
          uVar6 = CONCAT31(extraout_var,bVar1);
          if ((param_1 != (void *)0x0) &&
             (*(int *)(local_54 + 0x58) - *(int *)(local_54 + 0x5c) < (int)param_1)) {
            uStack_5c = 0xf3;
            uStack_58 = 1;
            local_4._0_1_ = 6;
            FUN_00520580(this_00,&uStack_5c);
            local_4 = CONCAT31(local_4._1_3_,4);
            FUN_00619730();
          }
        }
        local_4 = CONCAT31(local_4._1_3_,2);
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_005510b0(auStack_30);
      }
    }
    if (uVar6 != 0) {
      ExceptionList = pvStack_c;
      return uVar6;
    }
  }
  local_54 = 1;
  local_50 = 1;
  local_4 = 7;
  FUN_00520580(this_00,&local_54);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return 0;
}

