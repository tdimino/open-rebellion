
uint FUN_00560200(int *param_1,int *param_2,void *param_3)

{
  uint uVar1;
  uint uVar2;
  int *piVar3;
  uint *puVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  int *piVar8;
  int *piVar9;
  void *pvVar10;
  bool bVar11;
  undefined4 *puVar12;
  int local_60;
  int *piStack_5c;
  int local_58;
  int *piStack_54;
  int *local_50;
  uint auStack_4c [2];
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_30;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  piVar3 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a190;
  local_c = ExceptionList;
  local_60 = 0;
  local_58 = 0;
  ExceptionList = &local_c;
  puVar4 = FUN_004025b0(param_2,(uint *)&local_50);
  piVar9 = param_1;
  uVar1 = *puVar4;
  uVar2 = param_1[0x27];
  param_2 = (int *)(uint)(uVar1 == uVar2);
  FUN_00619730();
  piVar7 = param_2;
  if ((int *)(uint)(uVar1 == uVar2) != (int *)0x0) {
    puVar4 = FUN_004025b0(piVar9,(uint *)&param_1);
    uVar1 = *puVar4;
    uVar2 = piVar3[0x27];
    param_2 = (int *)(uint)(uVar1 == uVar2);
    FUN_00619730();
    piVar7 = param_2;
    if ((int *)(uint)(uVar1 == uVar2) != (int *)0x0) {
      iVar5 = FUN_004ed300((int)piVar3);
      if ((iVar5 == 0) || (iVar5 = FUN_004ed300((int)piVar9), iVar5 == 0)) {
        param_2 = (int *)0x0;
        piVar7 = param_2;
      }
      else {
        piVar7 = (int *)0x1;
      }
    }
  }
  if (piVar7 == (int *)0x0) {
    ExceptionList = local_c;
    return 0;
  }
  iVar5 = FUN_004ed300((int)piVar3);
  iVar6 = FUN_004ed300((int)piVar9);
  if (iVar5 != iVar6) {
    ExceptionList = local_c;
    return 0;
  }
  FUN_004fd450(&local_44,(int)param_3);
  local_4 = 0;
  local_40 = 1;
  local_30 = FUN_004ed300((int)piVar9);
  iVar5 = piVar9[0x25];
  puVar4 = FUN_004025b0(piVar3,(uint *)&param_2);
  puVar12 = &local_44;
  local_4._0_1_ = 1;
  iVar6 = (**(code **)(*piVar3 + 0x1f0))();
  iVar6 = FUN_005349e0(piVar9,puVar4,iVar6,puVar12);
  bVar11 = iVar6 != 0;
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_60 = (int)(short)piVar9[0x25] - (int)(short)iVar5;
  iVar5 = (**(code **)(*piVar3 + 0x1f0))();
  param_2 = (int *)(**(code **)(*piVar9 + 0x1f0))();
  if (local_60 == 0) {
    iVar6 = FUN_0053ea70((int)param_2,iVar5,&local_60);
    if ((iVar6 == 0) || (!bVar11)) {
      bVar11 = false;
    }
    else {
      bVar11 = true;
    }
    if (local_60 != 0) {
      iVar6 = (**(code **)(*piVar9 + 0x2f0))(local_60,10,&local_44);
      if ((iVar6 == 0) || (!bVar11)) {
        bVar11 = false;
      }
      else {
        bVar11 = true;
      }
    }
  }
  if (local_58 == 0) {
    iVar5 = FUN_0053ea70(iVar5,(int)param_2,&local_58);
    if ((iVar5 == 0) || (!bVar11)) {
      bVar11 = false;
    }
    else {
      bVar11 = true;
    }
    if (local_58 != 0) {
      iVar5 = (**(code **)(*piVar3 + 0x2f0))(local_58,10,&local_44);
      if ((iVar5 == 0) || (!bVar11)) {
        bVar11 = false;
      }
      else {
        bVar11 = true;
      }
    }
    if (local_58 != 0) goto LAB_0056042d;
  }
  else {
LAB_0056042d:
    iVar5 = FUN_005340a0(piVar9,*(short *)((int)piVar9 + 0x62) + DAT_006b9060,&local_44);
    if ((iVar5 == 0) || (!bVar11)) {
      bVar11 = false;
    }
    else {
      bVar11 = true;
    }
  }
  if (local_60 != 0) {
    iVar5 = FUN_005340a0(piVar3,*(short *)((int)piVar3 + 0x62) + DAT_006b9060,&local_44);
    if ((iVar5 == 0) || (!bVar11)) {
      bVar11 = false;
    }
    else {
      bVar11 = true;
    }
  }
  piVar7 = (int *)FUN_004ece30(&param_2);
  local_4._0_1_ = 2;
  iVar5 = FUN_004ee500(piVar9,piVar7,&local_44);
  if ((iVar5 == 0) || (!bVar11)) {
    bVar11 = false;
  }
  else {
    bVar11 = true;
  }
  local_4._0_1_ = 0;
  FUN_00619730();
  piVar7 = (int *)FUN_004ece30(&param_2);
  local_4._0_1_ = 3;
  iVar5 = FUN_004ee500(piVar3,piVar7,&local_44);
  if ((iVar5 == 0) || (!bVar11)) {
    bVar11 = false;
  }
  else {
    bVar11 = true;
  }
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_004ef1c0(piVar9,0);
  FUN_004ef1c0(piVar3,0);
  FUN_004fd450(&uStack_28,(int)&local_44);
  local_4._0_1_ = 4;
  uStack_24 = 1;
  uStack_20 = 3;
  iVar5 = FUN_004f9510(piVar9,1,&uStack_28);
  if ((iVar5 == 0) || (!bVar11)) {
    bVar11 = false;
  }
  else {
    bVar11 = true;
  }
  iVar5 = FUN_004f9510(piVar9,2,&uStack_28);
  if ((iVar5 == 0) || (!bVar11)) {
    bVar11 = false;
  }
  else {
    bVar11 = true;
  }
  iVar5 = FUN_004f9510(piVar3,1,&uStack_28);
  if ((iVar5 == 0) || (!bVar11)) {
    bVar11 = false;
  }
  else {
    bVar11 = true;
  }
  iVar5 = FUN_004f9510(piVar3,2,&uStack_28);
  if ((iVar5 == 0) || (param_2 = (int *)0x1, !bVar11)) {
    param_2 = (int *)0x0;
  }
  piVar8 = (int *)0x0;
  local_50 = (int *)0x0;
  piStack_54 = (int *)0x0;
  piStack_5c = (int *)0x0;
  FUN_004ece30(&param_1);
  local_4._0_1_ = 5;
  puVar4 = FUN_004025b0(piVar9,auStack_4c);
  local_4._0_1_ = 6;
  FUN_004f26d0(&param_1,puVar4);
  local_4 = CONCAT31(local_4._1_3_,5);
  FUN_00619730();
  bVar11 = param_1 != (int *)0x32000242;
  auStack_4c[0] = 0x32000242;
  FUN_00619730();
  piVar7 = piVar9;
  if (bVar11) {
    bVar11 = param_1 == (int *)0x31000241;
    auStack_4c[0] = 0x31000241;
    FUN_00619730();
    piVar7 = piVar8;
    if (bVar11) {
      local_50 = piVar9;
    }
    else {
      bVar11 = param_1 == (int *)0x35000281;
      auStack_4c[0] = 0x35000281;
      FUN_00619730();
      if (bVar11) {
        piStack_54 = piVar9;
      }
      else {
        bVar11 = param_1 == (int *)0x34000280;
        auStack_4c[0] = 0x34000280;
        FUN_00619730();
        if (bVar11) {
          piStack_5c = piVar9;
        }
      }
    }
  }
  puVar4 = FUN_004025b0(piVar3,auStack_4c);
  local_4._0_1_ = 7;
  FUN_004f26d0(&param_1,puVar4);
  local_4 = CONCAT31(local_4._1_3_,5);
  FUN_00619730();
  bVar11 = param_1 != (int *)0x32000242;
  auStack_4c[0] = 0x32000242;
  FUN_00619730();
  piVar9 = piVar3;
  if (bVar11) {
    bVar11 = param_1 == (int *)0x31000241;
    auStack_4c[0] = 0x31000241;
    FUN_00619730();
    piVar9 = piVar7;
    if (bVar11) {
      local_50 = piVar3;
    }
    else {
      bVar11 = param_1 == (int *)0x35000281;
      auStack_4c[0] = 0x35000281;
      FUN_00619730();
      if (bVar11) {
        piStack_54 = piVar3;
      }
      else {
        bVar11 = param_1 == (int *)0x34000280;
        auStack_4c[0] = 0x34000280;
        FUN_00619730();
        if (bVar11) {
          piStack_5c = piVar3;
        }
      }
    }
  }
  piVar3 = piStack_54;
  pvVar10 = param_3;
  if (piVar9 == (int *)0x0) goto LAB_005608e9;
  if (piStack_54 != (int *)0x0) {
    if ((*(byte *)(piVar9 + 0x2c) & 1) == 0) {
      iVar5 = FUN_0056f650(piVar9,1,&local_44);
      if ((iVar5 == 0) || (param_2 == (int *)0x0)) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = (int *)0x1;
      }
      pvVar10 = param_3;
      param_2 = piVar7;
      if ((*(byte *)(piVar9 + 0x2b) & 1) != 0) goto LAB_00560869;
      if ((short)piVar9[0x23] < DAT_006bb74c) {
        iVar5 = FUN_00561b60(piVar9,&local_44);
        if ((iVar5 == 0) || (piVar7 == (int *)0x0)) {
          param_2 = (int *)0x0;
        }
        else {
          param_2 = (int *)0x1;
        }
      }
    }
    piVar7 = param_2;
    iVar5 = DAT_006bb73c;
    pvVar10 = param_3;
    if ((*(byte *)(piVar9 + 0x2b) & 1) == 0) {
      iVar6 = (int)(short)piVar3[0x23] - (int)(short)piVar9[0x23];
      if (0 < iVar6) {
        iVar6 = FUN_0053e190(iVar6,DAT_006bb72c);
        iVar5 = FUN_0053e130(iVar5,iVar6);
      }
      pvVar10 = param_3;
      iVar5 = FUN_004ee150(piVar9,*(short *)((int)piVar9 + 0x8e) + iVar5,param_3);
      if ((iVar5 == 0) || (piVar7 == (int *)0x0)) {
        param_2 = (int *)0x0;
      }
      else {
        param_2 = (int *)0x1;
      }
    }
  }
LAB_00560869:
  piVar3 = param_2;
  iVar5 = DAT_006bb71c;
  if ((piStack_5c != (int *)0x0) && ((*(byte *)(piVar9 + 0x2b) & 1) == 0)) {
    iVar6 = (int)(short)piStack_5c[0x23] - (int)(short)piVar9[0x23];
    if (0 < iVar6) {
      iVar6 = FUN_0053e190(iVar6,DAT_006bb720);
      iVar5 = FUN_0053e130(iVar5,iVar6);
    }
    iVar5 = FUN_004ee150(piVar9,*(short *)((int)piVar9 + 0x8e) + iVar5,pvVar10);
    if ((iVar5 == 0) || (piVar3 == (int *)0x0)) {
      param_2 = (int *)0x0;
    }
    else {
      param_2 = (int *)0x1;
    }
  }
LAB_005608e9:
  piVar3 = local_50;
  piVar9 = param_2;
  iVar5 = DAT_006bb724;
  if (local_50 != (int *)0x0) {
    if (((piStack_54 != (int *)0x0) && ((local_50[0x2b] & 1U) == 0)) &&
       ((local_50[0x2b] & 0x10U) != 0)) {
      iVar6 = (int)(short)piStack_54[0x23] - (int)(short)local_50[0x23];
      if (0 < iVar6) {
        iVar6 = FUN_0053e190(iVar6,DAT_006bb728);
        iVar5 = FUN_0053e130(iVar5,iVar6);
      }
      iVar5 = FUN_004ee150(piVar3,*(short *)((int)piVar3 + 0x8e) + iVar5,pvVar10);
      if ((iVar5 == 0) || (piVar9 == (int *)0x0)) {
        param_2 = (int *)0x0;
      }
      else {
        param_2 = (int *)0x1;
      }
    }
    piVar9 = param_2;
    iVar5 = DAT_006bb738;
    if (((piStack_5c != (int *)0x0) && ((piVar3[0x2b] & 1U) == 0)) && ((piVar3[0x2b] & 0x10U) != 0))
    {
      iVar6 = (int)(short)piStack_5c[0x23] - (int)(short)piVar3[0x23];
      if (0 < iVar6) {
        iVar6 = FUN_0053e190(iVar6,DAT_006bb754);
        iVar5 = FUN_0053e130(iVar5,iVar6);
      }
      iVar5 = FUN_004ee150(piVar3,*(short *)((int)piVar3 + 0x8e) + iVar5,pvVar10);
      if ((iVar5 == 0) || (param_2 = (int *)0x1, piVar9 == (int *)0x0)) {
        param_2 = (int *)0x0;
      }
    }
  }
  local_4._0_1_ = 4;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004fd4d0(&uStack_28);
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_44);
  ExceptionList = local_c;
  return (uint)param_2;
}

