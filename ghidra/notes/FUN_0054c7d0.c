
int FUN_0054c7d0(int param_1,int *param_2,uint *param_3,uint *param_4,uint *param_5,void *param_6,
                void *param_7)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  int *piVar4;
  uint uVar5;
  void *pvVar6;
  uint *puVar7;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *this;
  void *pvVar8;
  int iVar9;
  uint uVar10;
  uint uVar11;
  uint *puVar12;
  undefined1 *puVar13;
  uint local_48;
  int local_44;
  undefined1 auStack_40 [4];
  uint uStack_3c;
  uint uStack_38;
  uint local_34;
  uint local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006480e8;
  pvStack_c = ExceptionList;
  iVar9 = 1;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_48);
  local_4 = 0;
  FUN_00536da0(local_2c,param_1,3);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00513120((int)local_2c);
  do {
    if (local_10 == 0) {
      local_4 = local_4 & 0xffffff00;
      FUN_00536ea0(local_2c);
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = pvStack_c;
      return iVar9;
    }
    iVar3 = FUN_0052bed0((int)local_2c);
    if (((*(uint *)(iVar3 + 0x50) >> 0xc & 1) != 0) ||
       (iVar3 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar3 + 0x78) >> 6 & 1) != 0)) {
      local_34 = 0x30;
      local_30 = 0x3c;
      local_4._0_1_ = 2;
      piVar4 = (int *)FUN_0052bed0((int)local_2c);
      uVar5 = (**(code **)(*piVar4 + 4))();
      if ((uVar5 < local_34) || (local_30 <= uVar5)) {
LAB_0054c8d2:
        bVar1 = false;
      }
      else {
        puVar13 = auStack_40;
        pvVar6 = (void *)FUN_0052bed0((int)local_2c);
        puVar7 = FUN_004ed270(pvVar6,puVar13);
        local_4._0_1_ = 3;
        iVar3 = FUN_004ece60(puVar7);
        local_4._0_1_ = 2;
        FUN_00619730();
        if (iVar3 == 0) goto LAB_0054c8d2;
        bVar1 = true;
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      bVar1 = !bVar1;
      if (bVar1) {
        piVar4 = &param_1;
        param_1 = 0;
        pvVar6 = (void *)FUN_0052bed0((int)local_2c);
        bVar2 = FUN_004f6b50(pvVar6,piVar4);
        if ((CONCAT31(extraout_var,bVar2) == 0) || (iVar9 == 0)) {
          iVar9 = 0;
        }
        else {
          iVar9 = 1;
        }
        if (param_1 != 0) {
          iVar3 = FUN_0052bed0((int)local_2c);
          uStack_38 = *(uint *)(iVar3 + 0x50) >> 3 & 1;
          if ((uStack_38 == 0) &&
             (((((*(byte *)(param_1 + 0x50) & 8) != 0 ||
                (iVar3 = FUN_0052bed0((int)local_2c),
                ((*(uint *)(iVar3 + 0x24) ^ *(uint *)(param_1 + 0x24)) & 0xc0) != 0)) &&
               ((*(uint *)(local_44 + 0x50) & 4) != 0)) &&
              ((*(uint *)(local_44 + 0x50) & 0x800) == 0)))) {
            iVar3 = FUN_0052bed0((int)local_2c);
            uVar10 = *(uint *)(iVar3 + 0x24) >> 6 & 3;
            iVar3 = FUN_0052bed0((int)local_2c);
            puVar7 = &local_48;
            puVar12 = (uint *)0x0;
            uVar11 = *(uint *)(iVar3 + 0x78) >> 6 & 1;
            piVar4 = (int *)FUN_0052bed0((int)local_2c);
            uVar5 = FUN_00543040(piVar4,puVar12,puVar7);
            if ((uVar5 == 0) || (iVar9 == 0)) {
              iVar9 = 0;
            }
            else {
              iVar9 = 1;
            }
            if (*param_2 == 0) {
              *param_2 = 1;
              *param_3 = uVar10;
              *param_4 = uVar11;
              FUN_004f26d0(param_5,&local_48);
            }
            if (((*param_3 == uVar10) && (*param_4 == uVar11)) && (*param_5 == local_48)) {
              puVar7 = &uStack_3c;
              pvVar6 = (void *)FUN_0052bed0((int)local_2c);
              puVar7 = FUN_004025b0(pvVar6,puVar7);
              local_4._0_1_ = 4;
              bVar2 = FUN_004f44b0(param_6,puVar7,0);
              if ((CONCAT31(extraout_var_00,bVar2) == 0) || (iVar9 == 0)) {
                iVar9 = 0;
              }
              else {
                iVar9 = 1;
              }
              local_4 = CONCAT31(local_4._1_3_,1);
              FUN_00619730();
            }
            else {
              bVar1 = false;
            }
          }
        }
      }
      pvVar6 = param_7;
      if (bVar1) {
        iVar3 = 0;
        pvVar8 = param_7;
        this = (void *)FUN_0052bed0((int)local_2c);
        iVar3 = FUN_004f78e0(this,iVar3,pvVar8);
        if ((iVar3 == 0) || (iVar9 == 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        iVar9 = 0;
        pvVar8 = (void *)FUN_0052bed0((int)local_2c);
        iVar9 = FUN_00534790(pvVar8,iVar9,pvVar6);
        if ((iVar9 == 0) || (!bVar1)) {
          iVar9 = 0;
        }
        else {
          iVar9 = 1;
        }
      }
    }
    FUN_005130d0((int)local_2c);
  } while( true );
}

