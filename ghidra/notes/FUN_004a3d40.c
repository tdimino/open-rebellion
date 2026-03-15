
void __thiscall FUN_004a3d40(void *this,int param_1,int *param_2)

{
  undefined4 uVar1;
  uint uVar2;
  bool bVar3;
  bool bVar4;
  byte bVar5;
  undefined4 uVar6;
  undefined4 *this_00;
  HGDIOBJ pvVar7;
  undefined *puVar8;
  char *pcVar9;
  undefined4 *puVar10;
  void *pvVar11;
  uint *puVar12;
  undefined3 extraout_var;
  uint *puVar13;
  void *this_01;
  int *piVar14;
  undefined3 extraout_var_00;
  uint *puVar15;
  uint uVar16;
  uint uVar17;
  int iVar18;
  undefined1 uVar19;
  int iVar20;
  undefined4 *local_84;
  uint uStack_7c;
  uint *local_74;
  void *local_70;
  uint local_6c;
  undefined4 *local_68;
  uint uStack_60;
  uint uStack_5c;
  uint uStack_58;
  uint uStack_54;
  uint uStack_50;
  uint uStack_4c;
  uint uStack_48;
  void *local_44;
  uint *local_40;
  uint *local_3c;
  HGDIOBJ pvStack_38;
  HGDIOBJ pvStack_34;
  HPALETTE local_30;
  undefined4 local_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637bfe;
  pvStack_c = ExceptionList;
  bVar4 = false;
  ExceptionList = &pvStack_c;
  uVar6 = FUN_006037f0(7);
  local_44 = (void *)*param_2;
  local_70 = (void *)0x0;
  FUN_00502db0(local_2c,param_1,3);
  uVar1 = *(undefined4 *)(param_1 + 0x24);
  local_4 = 0;
  uVar17 = *(uint *)(*param_2 + 0x68) >> 1 & 1;
  local_6c = 0;
  local_30 = (HPALETTE)FUN_00618b70(0x28);
  local_4._0_1_ = 1;
  if (local_30 == (HPALETTE)0x0) {
    local_68 = (undefined4 *)0x0;
  }
  else {
    local_68 = (undefined4 *)
               FUN_005fbd20(local_30,uVar6,(-(uint)(((byte)uVar1 & 0xc0) != 0x40) & 0x32) + 0x28a1,2
                           );
  }
  local_4._0_1_ = 0;
  puVar12 = *(uint **)(*param_2 + 0x20);
  puVar15 = *(uint **)(*param_2 + 0x24);
  local_30 = (HPALETTE)FUN_00618b70(0x28);
  local_4._0_1_ = 2;
  if (local_30 == (HPALETTE)0x0) {
    this_00 = (undefined4 *)0x0;
  }
  else {
    this_00 = (undefined4 *)
              FUN_005fbea0(local_30,0x5b,0x32,*(uint **)((int)this + 0xa4),
                           *(HDC *)((int)this + 0x1ac));
  }
  local_4._0_1_ = 0;
  FUN_005fcfe0(this_00,0,0,0x5b,0x32,0);
  local_40 = puVar12;
  local_3c = puVar15;
  pvVar7 = (HGDIOBJ)FUN_005fc130((int)this_00);
  pvStack_38 = SelectObject(*(HDC *)((int)this + 0x1ac),pvVar7);
  local_30 = SelectPalette(*(HDC *)((int)this + 0x1ac),DAT_006be5a0,0);
  pvStack_34 = SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a0));
  SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a8));
  puVar8 = FUN_004f62d0(param_1);
  pcVar9 = (char *)FUN_00583c40((int)puVar8);
  FUN_005f35e0((void *)(*param_2 + 0x14),pcVar9);
  if (uVar17 == 0) {
    FUN_005fcfe0(puVar12,0,0,3,0x32,(char)puVar12[2]);
    FUN_005fcfe0(puVar15,0,0,3,0x32,(char)puVar12[2]);
  }
  else {
    SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a0));
    Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a87c8,2);
    FUN_005fd0f0(this_00,puVar12,0,0);
    FUN_005fd0f0(this_00,puVar15,0,0);
  }
  FUN_00513120((int)local_2c);
  *param_2 = *(int *)(*param_2 + 8);
  while (iStack_10 != 0) {
    uStack_7c = 0;
    bVar3 = false;
    puVar10 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 3;
    if (puVar10 == (undefined4 *)0x0) {
      puVar10 = (undefined4 *)0x0;
    }
    else {
      puVar10 = FUN_004acd10(puVar10);
    }
    puVar12 = &uStack_60;
    local_4._0_1_ = 0;
    pvVar11 = (void *)FUN_0052bed0((int)local_2c);
    puVar12 = FUN_004025b0(pvVar11,puVar12);
    local_4._0_1_ = 4;
    FUN_004f26d0(puVar10 + 6,puVar12);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_004ace40(puVar10,*(uint *)((int)this + 0x24));
    FUN_004acba0((void *)((int)this + 0x114),puVar10,0);
    if (*param_2 == 0) {
      bVar3 = true;
    }
    else {
      puVar12 = &uStack_5c;
      pvVar11 = (void *)FUN_0052bed0((int)local_2c);
      puVar12 = FUN_004025b0(pvVar11,puVar12);
      uVar16 = *puVar12;
      uVar2 = *(uint *)(*param_2 + 0x6c);
      FUN_00619730();
      if (uVar2 == uVar16) {
        if (local_70 != (void *)0x0) {
          puVar12 = *(uint **)((int)local_70 + 0x20);
          puVar15 = *(uint **)((int)local_70 + 0x24);
          FUN_005fcfe0(this_00,0,0,0x5b,0x32,0);
          pvVar7 = (HGDIOBJ)FUN_005fc130((int)this_00);
          SelectObject(*(HDC *)((int)this + 0x1ac),pvVar7);
          SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a0));
          Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a87d8,2);
          FUN_005fd0f0(this_00,puVar12,0,0);
          FUN_005fd0f0(this_00,puVar15,0,0);
        }
        local_70 = (void *)*param_2;
        piVar14 = (int *)FUN_0052bed0((int)local_2c);
        bVar5 = FUN_004a66a0(piVar14);
        uStack_7c = CONCAT31(extraout_var,bVar5);
        if (uStack_7c != *(uint *)(*param_2 + 0x54)) {
          *(uint *)(*param_2 + 0x54) = uStack_7c;
          puVar12 = *(uint **)(*param_2 + 0x20);
          puVar15 = *(uint **)(*param_2 + 0x24);
          pvVar11 = (void *)FUN_0052bed0((int)local_2c);
          puVar13 = FUN_004025b0(pvVar11,&uStack_58);
          iVar20 = 1;
          iVar18 = 0;
          local_4._0_1_ = 5;
          piVar14 = (int *)FUN_0052bed0((int)local_2c);
          puVar13 = FUN_0042c3b0(*(undefined4 *)((int)this + 0x1b0),puVar13,piVar14,iVar18,iVar20);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005fcfe0(puVar12,0,0,0x5b,0x32,(char)puVar13[2]);
          FUN_005fd0f0(puVar13,puVar12,5,0xf);
          if (puVar13 != (uint *)0x0) {
            FUN_005fbfa0(puVar13);
            FUN_00618b60((undefined *)puVar13);
          }
          FUN_005fcfe0(this_00,0,0,0x5b,0x32,0);
          pvVar7 = (HGDIOBJ)FUN_005fc130((int)this_00);
          SelectObject(*(HDC *)((int)this + 0x1ac),pvVar7);
          SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a0));
          Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a87f8,2);
          Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a87e8,2);
          FUN_005fd0f0(this_00,puVar12,0,0);
          iVar18 = FUN_0052bed0((int)local_2c);
          FUN_004a67a0(puVar12,*(uint *)(iVar18 + 0x24) >> 6 & 3,uStack_7c,1);
          FUN_005fcc30(puVar12,puVar15,0,0,0,0,(undefined4 *)0x0,0);
          FUN_005fd0f0(local_68,puVar15,0,0);
        }
        iVar18 = FUN_0052bed0((int)local_2c);
        puVar8 = FUN_004f62d0(iVar18);
        pcVar9 = (char *)FUN_00583c40((int)puVar8);
        FUN_005f35e0((void *)(*param_2 + 0x14),pcVar9);
        uVar16 = *(uint *)(*param_2 + 0x68);
        if (uVar17 == 0) {
          uVar16 = uVar16 | 1;
        }
        else {
          uVar16 = uVar16 & 0xfffffffe;
        }
        *(uint *)(*param_2 + 0x68) = uVar16;
        *param_2 = *(int *)(*param_2 + 8);
        FUN_005130d0((int)local_2c);
      }
      else if ((*(byte *)(*param_2 + 0x68) & 4) == 0) {
        puVar12 = &uStack_54;
        pvVar11 = (void *)FUN_0052bed0((int)local_2c);
        puVar12 = FUN_004025b0(pvVar11,puVar12);
        uVar16 = *(uint *)(*param_2 + 0x6c);
        uVar2 = *puVar12;
        FUN_00619730();
        if (uVar2 < uVar16) {
          bVar3 = true;
        }
        else {
          puVar12 = &uStack_50;
          pvVar11 = (void *)FUN_0052bed0((int)local_2c);
          puVar12 = FUN_004025b0(pvVar11,puVar12);
          uVar16 = *puVar12;
          uVar2 = *(uint *)(*param_2 + 0x6c);
          FUN_00619730();
          if (uVar2 < uVar16) {
            iVar18 = *(int *)(*param_2 + 4);
            iVar20 = *(int *)(*param_2 + 8);
            *(int *)(iVar18 + 8) = iVar20;
            if (iVar20 != 0) {
              *(int *)(iVar20 + 4) = iVar18;
            }
            if ((undefined4 *)*param_2 != (undefined4 *)0x0) {
              (*(code *)**(undefined4 **)*param_2)(1);
            }
            *param_2 = iVar20;
          }
        }
      }
      else {
        bVar3 = true;
      }
    }
    if (bVar3) {
      if (local_70 != (void *)0x0) {
        puVar12 = *(uint **)((int)local_70 + 0x20);
        puVar15 = *(uint **)((int)local_70 + 0x24);
        FUN_005fcfe0(this_00,0,0,0x5b,0x32,0);
        pvVar7 = (HGDIOBJ)FUN_005fc130((int)this_00);
        SelectObject(*(HDC *)((int)this + 0x1ac),pvVar7);
        SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a0));
        Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a87d8,2);
        FUN_005fd0f0(this_00,puVar12,0,0);
        FUN_005fd0f0(this_00,puVar15,0,0);
      }
      pvVar11 = (void *)FUN_00618b70(0x70);
      local_4._0_1_ = 6;
      if (pvVar11 == (void *)0x0) {
        local_70 = (void *)0x0;
      }
      else {
        puVar12 = &uStack_4c;
        this_01 = (void *)FUN_0052bed0((int)local_2c);
        puVar12 = FUN_004025b0(this_01,puVar12);
        bVar4 = true;
        local_4 = CONCAT31(local_4._1_3_,7);
        local_70 = FUN_004c7e10(pvVar11,puVar12);
      }
      local_4 = 6;
      if (bVar4) {
        bVar4 = false;
        FUN_00619730();
      }
      local_4 = local_4 & 0xffffff00;
      FUN_0052bed0((int)local_2c);
      pvVar11 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 8;
      if (pvVar11 == (void *)0x0) {
        puVar12 = (uint *)0x0;
      }
      else {
        puVar12 = (uint *)FUN_005fbda0(pvVar11,0x5b,0x32,*(uint **)((int)this + 0xa4));
      }
      local_4._0_1_ = 0;
      pvVar11 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 9;
      if (pvVar11 == (void *)0x0) {
        local_74 = (uint *)0x0;
      }
      else {
        local_74 = (uint *)FUN_005fbda0(pvVar11,0x5b,0x32,*(uint **)((int)this + 0xa4));
      }
      local_4._0_1_ = 0;
      piVar14 = (int *)FUN_0052bed0((int)local_2c);
      bVar5 = FUN_004a66a0(piVar14);
      uStack_7c = CONCAT31(extraout_var_00,bVar5);
      *(uint *)((int)local_70 + 0x54) = uStack_7c;
      pvVar11 = (void *)FUN_0052bed0((int)local_2c);
      puVar15 = FUN_004025b0(pvVar11,&uStack_48);
      iVar20 = 1;
      iVar18 = 0;
      local_4._0_1_ = 10;
      piVar14 = (int *)FUN_0052bed0((int)local_2c);
      puVar15 = FUN_0042c3b0(*(undefined4 *)((int)this + 0x1b0),puVar15,piVar14,iVar18,iVar20);
      local_4._0_1_ = 0;
      FUN_00619730();
      FUN_005fcfe0(puVar12,0,0,0x5b,0x32,(char)puVar15[2]);
      FUN_005fd0f0(puVar15,puVar12,5,0xf);
      if (puVar15 != (uint *)0x0) {
        FUN_005fbfa0(puVar15);
        FUN_00618b60((undefined *)puVar15);
      }
      iVar18 = FUN_0052bed0((int)local_2c);
      if ((*(uint *)(iVar18 + 0x24) >> 6 & 3) == *(uint *)((int)this + 0x170)) {
        uVar16 = *(uint *)((int)local_70 + 0x68) | 8;
      }
      else {
        uVar16 = *(uint *)((int)local_70 + 0x68) & 0xfffffff7;
      }
      *(uint *)((int)local_70 + 0x68) = uVar16;
      FUN_005fcfe0(this_00,0,0,0x5b,0x32,0);
      pvVar7 = (HGDIOBJ)FUN_005fc130((int)this_00);
      SelectObject(*(HDC *)((int)this + 0x1ac),pvVar7);
      SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a0));
      Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a87f8,2);
      Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a87e8,2);
      FUN_005fd0f0(this_00,puVar12,0,0);
      iVar18 = FUN_0052bed0((int)local_2c);
      FUN_004a67a0(puVar12,*(uint *)(iVar18 + 0x24) >> 6 & 3,uStack_7c,1);
      SetTextColor(*(HDC *)((int)this + 0x1ac),0xffffff);
      iVar18 = FUN_0052bed0((int)local_2c);
      puVar8 = FUN_004f62d0(iVar18);
      pcVar9 = (char *)FUN_00583c40((int)puVar8);
      FUN_005f35e0((void *)((int)local_70 + 0x14),pcVar9);
      FUN_005fcc30(puVar12,local_74,0,0,0,0,(undefined4 *)0x0,0);
      FUN_005fd0f0(local_68,local_74,0,0);
      if (uVar17 == 0) {
        uVar16 = *(uint *)((int)local_70 + 0x68) | 1;
      }
      else {
        uVar16 = *(uint *)((int)local_70 + 0x68) & 0xfffffffe;
      }
      *(uint *)((int)local_70 + 0x68) = uVar16;
      FUN_0060be60(local_70,(int)puVar12,(int)local_74);
      if (*param_2 == 0) {
        FUN_005f59f0(*(void **)((int)this + 0x188),(int)local_70);
      }
      else {
        iVar18 = *(int *)(*param_2 + 4);
        *(void **)(iVar18 + 8) = local_70;
        *(void **)(*param_2 + 4) = local_70;
        *(int *)((int)local_70 + 8) = *param_2;
        *(int *)((int)local_70 + 4) = iVar18;
      }
      FUN_005130d0((int)local_2c);
    }
    local_6c = local_6c | uStack_7c;
  }
  if (local_6c != *(uint *)((int)local_44 + 0x54)) {
    *(uint *)((int)local_44 + 0x54) = local_6c;
    local_70 = (void *)0x0;
    if ((*(uint *)(param_1 + 0x24) >> 6 & 3) == 1) {
      local_44 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 0xb;
      if (local_44 == (void *)0x0) {
        local_84 = (undefined4 *)0x0;
      }
      else {
        local_84 = (undefined4 *)FUN_005fbd20(local_44,uVar6,0x28a0,2);
      }
      local_4._0_1_ = 0;
      local_44 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 0xc;
      if (local_44 == (void *)0x0) {
        local_74 = (undefined4 *)0x0;
      }
      else {
        local_74 = (uint *)FUN_005fbd20(local_44,uVar6,0x28a1,2);
      }
    }
    else {
      local_70 = (void *)0x32;
      local_44 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 0xd;
      if (local_44 == (void *)0x0) {
        local_74 = (undefined4 *)0x0;
      }
      else {
        local_74 = (uint *)FUN_005fbd20(local_44,uVar6,0x28d3,2);
      }
      local_4._0_1_ = 0;
      local_44 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 0xe;
      if (local_44 == (void *)0x0) {
        local_84 = (undefined4 *)0x0;
      }
      else {
        local_84 = (undefined4 *)FUN_005fbd20(local_44,uVar6,0x28d2,2);
      }
    }
    local_4._0_1_ = 0;
    FUN_005fcfe0(this_00,0,0,0x5b,0x32,0);
    puVar12 = local_40;
    uVar19 = *(undefined1 *)(local_84 + 2);
    iVar18 = FUN_005fc0f0((int *)local_40);
    uVar17 = FUN_005fc0e0((int *)puVar12);
    FUN_005fcfe0(puVar12,0,0,uVar17,iVar18,uVar19);
    pvVar7 = (HGDIOBJ)FUN_005fc130((int)this_00);
    SelectObject(*(HDC *)((int)this + 0x1ac),pvVar7);
    SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a0));
    if ((*(byte *)(param_1 + 0x50) & 0x10) != 0) {
      local_40 = (uint *)FUN_00618b70(0x28);
      local_4._0_1_ = 0xf;
      if (local_40 == (uint *)0x0) {
        puVar10 = (undefined4 *)0x0;
      }
      else {
        puVar10 = (undefined4 *)FUN_005fbd20(local_40,uVar6,(int)local_70 + 0x28b7,2);
      }
      local_4._0_1_ = 0;
      FUN_005fd0f0(puVar10,puVar12,5,5);
      if (puVar10 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar10);
        FUN_00618b60((undefined *)puVar10);
      }
    }
    if ((*(uint *)(param_1 + 0x50) & 0x200) != 0) {
      local_40 = (uint *)FUN_00618b70(0x28);
      local_4._0_1_ = 0x10;
      if (local_40 == (uint *)0x0) {
        puVar10 = (undefined4 *)0x0;
      }
      else {
        puVar10 = (undefined4 *)FUN_005fbd20(local_40,uVar6,(int)local_70 + 0x28b8,2);
      }
      local_4._0_1_ = 0;
      FUN_005fd0f0(puVar10,puVar12,5,5);
      if (puVar10 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar10);
        FUN_00618b60((undefined *)puVar10);
      }
    }
    if (local_84 != (undefined4 *)0x0) {
      FUN_005fd0f0(local_84,puVar12,5,5);
      FUN_005fbfa0(local_84);
      FUN_00618b60((undefined *)local_84);
    }
    Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a8808,2);
    FUN_005fd0f0(this_00,puVar12,0,0);
    FUN_004a67a0(puVar12,*(uint *)(param_1 + 0x24) >> 6 & 3,local_6c,1);
    puVar15 = local_3c;
    FUN_005fcc30(puVar12,local_3c,0,0,0,0,(undefined4 *)0x0,0);
    FUN_005fd0f0(local_74,puVar15,0,0);
    if (local_74 != (undefined4 *)0x0) {
      FUN_005fbfa0(local_74);
      FUN_00618b60((undefined *)local_74);
    }
  }
  SelectObject(*(HDC *)((int)this + 0x1ac),pvStack_38);
  SelectObject(*(HDC *)((int)this + 0x1ac),pvStack_34);
  SelectObject(*(HDC *)((int)this + 0x1ac),local_30);
  if (this_00 != (undefined4 *)0x0) {
    FUN_005fbfa0(this_00);
    FUN_00618b60((undefined *)this_00);
  }
  if (local_68 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_68);
    FUN_00618b60((undefined *)local_68);
  }
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  ExceptionList = pvStack_c;
  return;
}

