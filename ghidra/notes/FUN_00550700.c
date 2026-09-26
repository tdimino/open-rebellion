
int __thiscall
FUN_00550700(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 *param_4
            ,int *param_5)

{
  bool bVar1;
  int *this_00;
  uint *puVar2;
  uint uVar3;
  undefined *puVar4;
  int iVar5;
  int iVar6;
  undefined4 uVar7;
  int *piVar8;
  void **ppvVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  uint local_54;
  void *pvStack_50;
  undefined4 uStack_4c;
  void *local_48;
  int iStack_44;
  uint local_40;
  uint local_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  void *pvStack_24;
  undefined4 uStack_20;
  undefined1 auStack_1c [8];
  undefined1 auStack_14 [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_5;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648a88;
  pvStack_c = ExceptionList;
  iVar6 = 1;
  ExceptionList = &pvStack_c;
  local_48 = this;
  puVar2 = FUN_004025b0(param_5,(uint *)&param_5);
  local_4 = 0;
  FUN_004f26d0((void *)((int)this + 0x18),puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_004ece30(&local_54);
  local_40 = 0xf1;
  local_3c = 0xf2;
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  uVar3 = (**(code **)(*this_00 + 4))();
  if ((uVar3 < local_40) || (local_3c <= uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = CONCAT31(local_4._1_3_,1);
  param_5 = (int *)CONCAT31(param_5._1_3_,!bVar1);
  FUN_00619730();
  if ((char)param_5 != '\0') {
    iVar6 = FUN_004f6b70(this_00,&local_54);
  }
  if (iVar6 != 0) {
    FUN_004f26d0((void *)((int)this + 0x20),&local_54);
    if (iVar6 != 0) {
      iVar5 = 0;
      param_5 = (int *)((int)local_48 + 0x2c);
      do {
        if (8 < iVar5) break;
        pvStack_50 = (void *)0x1;
        uStack_4c = 0;
        uStack_38 = 1;
        uStack_34 = 0xffffffff;
        local_40 = 1;
        local_3c = 0xffffffff;
        iVar6 = *this_00;
        puVar2 = &local_40;
        local_4 = CONCAT31(local_4._1_3_,4);
        puVar4 = FUN_0054f5b0(iVar5);
        iVar6 = (**(code **)(iVar6 + 0x74))(param_1,param_2,param_3,puVar4,puVar2);
        if (iVar6 != 0) {
          if (local_3c == 0xffffffff) {
            puVar11 = &uStack_38;
            puVar10 = &uStack_4c;
            iVar6 = *this_00;
            ppvVar9 = &pvStack_50;
            piVar8 = &iStack_44;
            uVar7 = *param_4;
            puVar4 = FUN_0054f5b0(iVar5);
            iVar6 = (**(code **)(iVar6 + 0x78))(puVar4,uVar7,piVar8,ppvVar9,puVar10,puVar11);
          }
          if (iVar6 != 0) {
            if ((iStack_44 == 0) && (local_3c == 0xffffffff)) {
              uStack_30 = 1;
              uStack_2c = 0x27;
              local_4._0_1_ = 5;
              FUN_00520580(&local_40,&uStack_30);
              local_4 = CONCAT31(local_4._1_3_,4);
              FUN_00619730();
            }
            if (iVar6 != 0) {
              FUN_0054f830(&uStack_28);
              local_4._0_1_ = 6;
              pvStack_24 = pvStack_50;
              uStack_20 = uStack_4c;
              FUN_00520580(auStack_14,&uStack_38);
              FUN_00520580(auStack_1c,&local_40);
              FUN_0054fae0(param_5,(int)&uStack_28);
              local_4 = CONCAT31(local_4._1_3_,4);
              FUN_0054f880(&uStack_28);
            }
          }
        }
        local_4._0_1_ = 3;
        FUN_00619730();
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        iVar5 = iVar5 + 1;
        param_5 = param_5 + 7;
      } while (iVar6 != 0);
      if (iVar6 != 0) {
        iVar5 = 0;
        param_5 = (int *)((int)local_48 + 300);
        do {
          if (8 < iVar5) break;
          local_48 = (void *)0x1;
          uStack_4c = 0;
          uStack_38 = 1;
          uStack_34 = 0xffffffff;
          local_40 = 1;
          local_3c = 0xffffffff;
          iVar6 = *this_00;
          puVar2 = &local_40;
          local_4 = CONCAT31(local_4._1_3_,8);
          puVar4 = FUN_0054f5b0(iVar5);
          iVar6 = (**(code **)(iVar6 + 0x7c))(param_1,param_2,param_3,puVar4,puVar2);
          if (iVar6 != 0) {
            if (local_3c == 0xffffffff) {
              puVar11 = &uStack_38;
              puVar10 = &uStack_4c;
              iVar6 = *this_00;
              ppvVar9 = &local_48;
              piVar8 = &iStack_44;
              uVar7 = *param_4;
              puVar4 = FUN_0054f5b0(iVar5);
              iVar6 = (**(code **)(iVar6 + 0x80))(puVar4,uVar7,piVar8,ppvVar9,puVar10,puVar11);
            }
            if (iVar6 != 0) {
              if ((iStack_44 == 0) && (local_3c == 0xffffffff)) {
                uStack_30 = 1;
                uStack_2c = 0x27;
                local_4._0_1_ = 9;
                FUN_00520580(&local_40,&uStack_30);
                local_4 = CONCAT31(local_4._1_3_,8);
                FUN_00619730();
              }
              if (iVar6 != 0) {
                FUN_0054f830(&uStack_28);
                pvStack_24 = local_48;
                local_4._0_1_ = 10;
                uStack_20 = uStack_4c;
                FUN_00520580(auStack_14,&uStack_38);
                FUN_00520580(auStack_1c,&local_40);
                FUN_0054fae0(param_5,(int)&uStack_28);
                local_4 = CONCAT31(local_4._1_3_,8);
                FUN_0054f880(&uStack_28);
              }
            }
          }
          local_4._0_1_ = 7;
          FUN_00619730();
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
          iVar5 = iVar5 + 1;
          param_5 = param_5 + 7;
        } while (iVar6 != 0);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar6;
}

