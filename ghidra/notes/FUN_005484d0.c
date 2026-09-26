
bool __thiscall FUN_005484d0(void *this,int *param_1,int param_2,int param_3)

{
  uint uVar1;
  bool bVar2;
  int *piVar3;
  bool bVar4;
  uint uVar5;
  int iVar6;
  undefined3 extraout_var;
  uint *puVar7;
  undefined4 uVar8;
  undefined3 extraout_var_00;
  int iVar9;
  int iVar10;
  bool bVar11;
  uint uStack_5c;
  uint uStack_58;
  uint local_54;
  uint local_50;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  piVar3 = param_1;
  puStack_8 = &LAB_00647ab8;
  pvStack_c = ExceptionList;
  bVar11 = true;
  local_54 = 0x56;
  local_50 = 0x57;
  iVar10 = 0;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  uVar5 = (**(code **)(*param_1 + 4))();
  if ((uVar5 < local_54) || (local_50 <= uVar5)) {
    uStack_5c = 0x57;
    uStack_58 = 0x58;
    local_4._0_1_ = 1;
    uVar5 = (**(code **)(*piVar3 + 4))();
    if ((uVar5 < uStack_5c) || (uStack_58 <= uVar5)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if (!bVar2) {
      bVar2 = false;
      goto LAB_0054856b;
    }
  }
  bVar2 = true;
LAB_0054856b:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar2) {
    if ((param_2 < 8) || (10 < param_2)) {
      iVar9 = 0;
    }
    else {
      iVar9 = 1;
    }
    if ((param_3 < 8) || (10 < param_3)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    if (iVar9 != iVar6) {
      param_1 = (int *)0x0;
      FUN_00521070(piVar3,(int *)&param_1);
      bVar11 = param_1 != (int *)0x0;
      if (param_1 != (int *)0x0) {
        local_54 = 0x90;
        local_50 = 0x98;
        local_4 = 2;
        uVar5 = (**(code **)(*param_1 + 4))();
        if ((uVar5 < local_54) || (local_50 <= uVar5)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        local_4 = 0xffffffff;
        FUN_00619730();
        piVar3 = param_1;
        if (bVar2) {
          param_2 = 0;
          param_3 = 0;
          bVar2 = false;
          FUN_00571c20(auStack_2c,this);
          local_4 = 3;
          FUN_00513120((int)auStack_2c);
          while (iStack_10 != 0) {
            iVar9 = FUN_0052bed0((int)auStack_2c);
            bVar4 = FUN_00520b30(iVar9);
            if (CONCAT31(extraout_var,bVar4) != 0) {
              iVar9 = FUN_0052bed0((int)auStack_2c);
              puVar7 = FUN_004025b0(piVar3,&uStack_5c);
              uVar5 = *puVar7;
              uVar1 = *(uint *)(iVar9 + 0x78);
              FUN_00619730();
              if (uVar5 == uVar1) {
                bVar2 = true;
                uVar8 = FUN_0052bed0((int)auStack_2c);
                iVar9 = FUN_00520cd0(uVar8);
                iVar9 = FUN_0055c620(iVar9);
                iVar10 = iVar10 + iVar9;
              }
            }
            FUN_005130d0((int)auStack_2c);
          }
          FUN_00569e10(auStack_4c,this);
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_00513120((int)auStack_4c);
          while (iStack_30 != 0) {
            iVar9 = FUN_0052bed0((int)auStack_4c);
            bVar4 = FUN_00520b30(iVar9);
            if (CONCAT31(extraout_var_00,bVar4) != 0) {
              iVar9 = FUN_0052bed0((int)auStack_4c);
              puVar7 = FUN_004025b0(piVar3,&uStack_5c);
              uVar5 = *puVar7;
              uVar1 = *(uint *)(iVar9 + 0x78);
              FUN_00619730();
              if (uVar5 == uVar1) {
                uVar8 = FUN_0052bed0((int)auStack_4c);
                iVar9 = FUN_00520cd0(uVar8);
                iVar9 = FUN_0055c630(iVar9);
                param_2 = param_2 + iVar9;
              }
            }
            FUN_005130d0((int)auStack_4c);
          }
          if (bVar2) {
            param_3 = FUN_0055c640();
          }
          iVar10 = FUN_0050cf10(piVar3,iVar10);
          if ((iVar10 == 0) || (!bVar11)) {
            bVar11 = false;
          }
          else {
            bVar11 = true;
          }
          iVar10 = FUN_0050cf30(piVar3,param_2);
          if ((iVar10 == 0) || (!bVar11)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          iVar10 = FUN_0050cf50(piVar3,param_3);
          if ((iVar10 == 0) || (bVar11 = true, !bVar2)) {
            bVar11 = false;
          }
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00569eb0(auStack_4c);
          local_4 = 0xffffffff;
          FUN_00571cc0(auStack_2c);
        }
      }
    }
  }
  ExceptionList = pvStack_c;
  return bVar11;
}

