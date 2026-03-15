
uint __thiscall FUN_00546370(void *this,uint *param_1,undefined4 param_2,uint param_3,void *param_4)

{
  uint uVar1;
  void *pvVar2;
  bool bVar3;
  bool bVar4;
  int *piVar5;
  undefined3 extraout_var;
  int iVar6;
  int iVar7;
  undefined4 *puVar8;
  undefined3 extraout_var_00;
  uint *puVar9;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  uint uVar10;
  undefined4 uVar11;
  void *pvVar12;
  undefined3 extraout_var_04;
  void *this_00;
  uint uVar13;
  int *piVar14;
  uint uStack_74;
  int *local_70;
  uint local_6c;
  uint local_68;
  int *local_64;
  int local_60;
  uint uStack_5c;
  int iStack_58;
  uint local_54;
  int *local_50;
  uint local_4c;
  uint uStack_48;
  undefined1 auStack_44 [4];
  uint uStack_40;
  uint uStack_3c;
  void *local_38;
  uint uStack_34;
  uint uStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006477a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_38 = this;
  piVar5 = (int *)FUN_00505190(param_1);
  uVar13 = (uint)(piVar5 != (int *)0x0);
  local_60 = 0;
  local_70 = piVar5;
  if (uVar13 != 0) {
    local_60 = thunk_FUN_00506e60();
    uVar13 = (uint)(local_60 != 0);
  }
  iVar7 = local_60;
  bVar3 = FUN_005438a0((int *)param_1);
  if (CONCAT31(extraout_var,bVar3) != 0) {
    iVar6 = FUN_00549910(this,param_4);
    if ((iVar6 == 0) || (uVar13 == 0)) {
      uVar13 = 0;
    }
    else {
      uVar13 = 1;
    }
  }
  if (piVar5 == (int *)0x0) {
    ExceptionList = local_c;
    return uVar13;
  }
  if (iVar7 == 0) {
    ExceptionList = local_c;
    return uVar13;
  }
  if ((((piVar5[0x14] & 8U) != 0) || ((int)(short)piVar5[0x26] != param_3)) ||
     ((piVar5[0x14] & 0x1000U) == 0)) goto LAB_00546cd2;
  FUN_004ed270(piVar5,&local_54);
  local_4 = 0;
  iVar7 = FUN_004f78e0(piVar5,0,param_4);
  if ((iVar7 == 0) || (uVar13 == 0)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  puVar8 = (undefined4 *)FUN_004ece30(&param_3);
  local_4._0_1_ = 1;
  bVar4 = FUN_004ef190(piVar5,puVar8);
  if ((CONCAT31(extraout_var_00,bVar4) == 0) || (!bVar3)) {
    uVar13 = 0;
  }
  else {
    uVar13 = 1;
  }
  local_4._0_1_ = 0;
  FUN_00619730();
  piVar14 = (int *)0x0;
  local_50 = (int *)0x0;
  FUN_0042d170(piVar5,&local_68);
  local_4 = CONCAT31(local_4._1_3_,2);
  puVar9 = FUN_004ece40(&local_68);
  if (puVar9 != (uint *)0x0) {
    piVar14 = (int *)FUN_00505970(&local_68);
    uVar13 = (uint)(piVar14 != (int *)0x0);
    local_50 = piVar14;
  }
  bVar3 = true;
  if (piVar14 != (int *)0x0) {
    if ((*(byte *)(local_70 + 0x2b) & 1) == 0) {
      puVar9 = FUN_004025b0(local_70,&local_4c);
      local_4._0_1_ = 4;
      bVar4 = FUN_00520bd0(piVar14,puVar9);
      if (CONCAT31(extraout_var_02,bVar4) == 0) {
        puVar9 = FUN_004025b0(local_70,&param_3);
        local_4._0_1_ = 5;
        bVar4 = FUN_00520bf0(piVar14,puVar9);
        local_4._0_1_ = 4;
        FUN_00619730();
        if (CONCAT31(extraout_var_03,bVar4) != 0) goto LAB_005465a8;
        bVar4 = false;
      }
      else {
LAB_005465a8:
        bVar4 = true;
      }
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
      if (!bVar4) goto LAB_005465c1;
    }
    else {
      puVar9 = FUN_004025b0(local_70,&param_3);
      local_4._0_1_ = 3;
      bVar4 = FUN_00520c10(piVar14,puVar9);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
      if (CONCAT31(extraout_var_01,bVar4) == 0) goto LAB_005465c1;
    }
    bVar3 = false;
  }
LAB_005465c1:
  if (bVar3) {
    piVar5 = (int *)0x0;
    iVar7 = FUN_004ece60(&local_54);
    if (iVar7 != 0) {
      piVar5 = (int *)FUN_00504dc0(&local_54);
      if ((piVar5 == (int *)0x0) || (uVar13 == 0)) {
        uVar13 = 0;
      }
      else {
        uVar13 = 1;
      }
      if (piVar5 != (int *)0x0) {
        if ((*(byte *)(piVar5 + 0x14) & 8) != 0) {
          piVar5 = (int *)0x0;
        }
        if (piVar5 != (int *)0x0) {
          FUN_004ece30(&param_3);
          local_4._0_1_ = 6;
          (**(code **)(*piVar5 + 0xc))(&param_3);
          iVar7 = FUN_004ece60(&param_3);
          if (iVar7 == 0) {
            piVar5 = (int *)0x0;
          }
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_00619730();
        }
      }
    }
    local_64 = (int *)0x0;
    FUN_004ece30(&local_6c);
    local_4._0_1_ = 7;
    FUN_004ece30(&iStack_58);
    local_4._0_1_ = 8;
    if (piVar5 != (int *)0x0) {
      uStack_40 = 0x30;
      uStack_3c = 0x40;
      local_4._0_1_ = 9;
      uVar10 = (**(code **)(*piVar5 + 4))();
      if ((uVar10 < uStack_40) || (uStack_3c <= uVar10)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      local_4._0_1_ = 8;
      FUN_00619730();
      if ((bVar3) && (iVar7 = (**(code **)(*piVar5 + 0x1d4))(), iVar7 == 0)) {
        puVar9 = (uint *)FUN_0042d170(piVar5,&param_3);
        local_4._0_1_ = 10;
        iVar7 = FUN_004ece60(puVar9);
        local_4._0_1_ = 8;
        FUN_00619730();
        if (iVar7 != 0) {
          puVar9 = (uint *)FUN_0042d170(piVar5,auStack_44);
          local_4._0_1_ = 0xb;
          piVar14 = (int *)FUN_00505970(puVar9);
          local_4._0_1_ = 8;
          local_64 = piVar14;
          FUN_00619730();
          if ((piVar14 == (int *)0x0) || (uVar13 == 0)) {
            uVar13 = 0;
          }
          else {
            uVar13 = 1;
          }
          if (piVar14 != (int *)0x0) {
            puVar8 = (undefined4 *)FUN_0042d170(piVar5,&param_3);
            local_4._0_1_ = 0xc;
            FUN_004f26d0(&local_6c,puVar8);
            local_4._0_1_ = 8;
            FUN_00619730();
            puVar8 = (undefined4 *)FUN_00535760(piVar5,&param_3);
            local_4._0_1_ = 0xd;
            FUN_004f26d0(&iStack_58,puVar8);
            local_4._0_1_ = 8;
            FUN_00619730();
          }
        }
      }
    }
    FUN_004ece30(&uStack_5c);
    local_4._0_1_ = 0xe;
    iVar7 = FUN_004f6b70(local_70,&uStack_5c);
    if ((iVar7 == 0) || (param_3 = 1, uVar13 == 0)) {
      param_3 = 0;
    }
    FUN_004ece30(&uStack_74);
    local_4._0_1_ = 0xf;
    if (piVar5 != (int *)0x0) {
      uStack_34 = 0x30;
      uStack_30 = 0x40;
      local_4._0_1_ = 0x10;
      uVar13 = (**(code **)(*piVar5 + 4))();
      if ((uVar13 < uStack_34) || (uStack_30 <= uVar13)) {
        uStack_40 = 0x1c;
        uStack_3c = 0x20;
        local_4._0_1_ = 0x11;
        uVar13 = (**(code **)(*piVar5 + 4))();
        if ((uVar13 < uStack_40) || (uStack_3c <= uVar13)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        local_4._0_1_ = 0x10;
        FUN_00619730();
        if (bVar3) goto LAB_005468e7;
        local_4c = 0x10;
        uStack_48 = 0x14;
        local_4._0_1_ = 0x12;
        uVar13 = (**(code **)(*piVar5 + 4))();
        if ((uVar13 < local_4c) || (uStack_48 <= uVar13)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        local_4._0_1_ = 0x10;
        FUN_00619730();
        if (bVar3) goto LAB_005468e7;
        bVar3 = false;
      }
      else {
LAB_005468e7:
        bVar3 = true;
      }
      local_4._0_1_ = 0xf;
      FUN_00619730();
      if (bVar3) {
        iVar7 = FUN_004f6b70(piVar5,&uStack_74);
        if ((iVar7 == 0) || (param_3 == 0)) {
          param_3 = 0;
        }
        else {
          param_3 = 1;
        }
      }
      else {
        uStack_34 = 0x14;
        uStack_30 = 0x1c;
        local_4._0_1_ = 0x13;
        uVar13 = (**(code **)(*piVar5 + 4))();
        if ((uVar13 < uStack_34) || (uStack_30 <= uVar13)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        local_4._0_1_ = 0xf;
        FUN_00619730();
        if (bVar3) {
          puVar9 = FUN_004025b0(piVar5,&local_4c);
          local_4._0_1_ = 0x14;
          FUN_004f26d0(&uStack_74,puVar9);
          local_4._0_1_ = 0xf;
          FUN_00619730();
        }
        else {
          param_3 = 0;
        }
      }
    }
    uVar13 = param_3;
    bVar3 = false;
    local_4c = 0;
    puVar9 = FUN_004ece40(&uStack_74);
    if ((puVar9 != (uint *)0x0) && (uStack_5c != uStack_74)) {
      puVar9 = FUN_004ece40(&local_68);
      if (puVar9 == (uint *)0x0) {
        puVar9 = FUN_004ece40(&local_6c);
        if (puVar9 != (uint *)0x0) {
          local_4c = 1;
        }
      }
      else {
        bVar3 = true;
      }
    }
    piVar5 = local_70;
    if (bVar3) {
      puVar9 = FUN_004025b0(local_70,&uStack_40);
      local_4._0_1_ = 0x15;
      iVar7 = FUN_00556390(puVar9,&uStack_74,param_4);
      if ((iVar7 == 0) || (uVar13 == 0)) {
        uVar13 = 0;
      }
      else {
        uVar13 = 1;
      }
      local_4._0_1_ = 0xf;
      param_3 = uVar13;
      FUN_00619730();
    }
    uVar11 = FUN_004ece60(&local_6c);
    puVar9 = FUN_004ece40(&local_68);
    if (puVar9 != (uint *)0x0) {
      puVar9 = FUN_004025b0(piVar5,&uStack_40);
      local_4._0_1_ = 0x16;
      iVar7 = (**(code **)(*local_50 + 0x1d0))(puVar9,uVar11,param_4);
      if ((iVar7 == 0) || (uVar13 == 0)) {
        uVar13 = 0;
      }
      else {
        uVar13 = 1;
      }
      local_4._0_1_ = 0xf;
      param_3 = uVar13;
      FUN_00619730();
    }
    puVar9 = FUN_004ece40(&local_6c);
    piVar14 = local_64;
    if (puVar9 != (uint *)0x0) {
      puVar9 = FUN_004025b0(piVar5,&param_3);
      piVar14 = local_64;
      local_4._0_1_ = 0x17;
      iVar7 = (**(code **)(*local_64 + 0x1cc))(puVar9,0,param_4);
      if ((iVar7 == 0) || (uVar13 == 0)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      local_4._0_1_ = 0xf;
      FUN_00619730();
      iVar7 = FUN_00534390(piVar5,&iStack_58,param_4);
      if ((iVar7 == 0) || (!bVar3)) {
        uVar13 = 0;
        param_3 = 0;
      }
      else {
        uVar13 = 1;
        param_3 = 1;
      }
    }
    if (local_4c != 0) {
      puVar9 = FUN_004025b0(piVar5,&uStack_40);
      local_4._0_1_ = 0x18;
      iVar7 = FUN_00556390(puVar9,&uStack_74,param_4);
      if ((iVar7 == 0) || (uVar13 == 0)) {
        uVar13 = 0;
      }
      else {
        uVar13 = 1;
      }
      local_4._0_1_ = 0xf;
      param_3 = uVar13;
      FUN_00619730();
    }
    puVar9 = FUN_004ece40(&local_6c);
    if ((((puVar9 != (uint *)0x0) && ((*(byte *)(piVar5 + 0x2b) & 1) != 0)) && (piVar5[7] != 0)) &&
       (((piVar5[9] ^ *(uint *)(piVar5[7] + 0x24)) & 0xc0) == 0)) {
      if (piVar14 == (int *)0x0) {
LAB_00546bfc:
        bVar3 = false;
      }
      else {
        uStack_34 = 0x62;
        uStack_30 = 99;
        local_4._0_1_ = 0x19;
        uVar10 = (**(code **)(*piVar14 + 4))();
        if ((uVar10 < uStack_34) || (uStack_30 <= uVar10)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        local_4._0_1_ = 0xf;
        FUN_00619730();
        if (!bVar3) goto LAB_00546bfc;
        puVar9 = FUN_004025b0(piVar5,&uStack_40);
        uVar10 = piVar14[0x1c];
        uVar1 = *puVar9;
        FUN_00619730();
        if (uVar10 != uVar1) goto LAB_00546bfc;
        bVar3 = true;
      }
      if (!bVar3) {
        puVar9 = FUN_004025b0(piVar5,&uStack_40);
        local_4._0_1_ = 0x1a;
        iVar7 = (**(code **)(*piVar14 + 0x1d0))(puVar9,0,param_4);
        if ((iVar7 == 0) || (param_3 = 1, uVar13 == 0)) {
          param_3 = 0;
        }
        local_4._0_1_ = 0xf;
        FUN_00619730();
      }
    }
    local_4._0_1_ = 0xe;
    FUN_00619730();
    local_4._0_1_ = 8;
    FUN_00619730();
    local_4._0_1_ = 7;
    FUN_00619730();
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00619730();
    uVar13 = param_3;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  this = local_38;
  piVar5 = local_70;
LAB_00546cd2:
  pvVar2 = param_4;
  iVar7 = (**(code **)(*piVar5 + 0xb0))(param_4);
  if ((iVar7 == 0) || (uVar13 == 0)) {
    uVar13 = 0;
  }
  else {
    uVar13 = 1;
  }
  FUN_00587100(auStack_2c,this);
  local_4 = 0x1b;
  FUN_00513120((int)auStack_2c);
  while (iStack_10 != 0) {
    puVar9 = param_1;
    pvVar12 = (void *)FUN_0052bed0((int)auStack_2c);
    bVar3 = FUN_00520cb0(pvVar12,(int *)puVar9);
    if (CONCAT31(extraout_var_04,bVar3) != 0) {
      pvVar12 = pvVar2;
      this_00 = (void *)FUN_0052bed0((int)auStack_2c);
      iVar7 = FUN_00522480(this_00,pvVar12);
      if ((iVar7 == 0) || (uVar13 == 0)) {
        uVar13 = 0;
      }
      else {
        uVar13 = 1;
      }
    }
    FUN_005130d0((int)auStack_2c);
  }
  local_4 = 0xffffffff;
  FUN_005871a0(auStack_2c);
  ExceptionList = local_c;
  return uVar13;
}

