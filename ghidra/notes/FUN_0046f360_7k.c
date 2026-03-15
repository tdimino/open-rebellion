
void __fastcall FUN_0046f360(void *param_1)

{
  undefined4 *this;
  bool bVar1;
  undefined1 uVar2;
  uint *puVar3;
  int *piVar4;
  HDC pHVar5;
  HDC hdc;
  char *pcVar6;
  int iVar7;
  int iVar8;
  HGDIOBJ pvVar9;
  void *pvVar10;
  undefined4 *puVar11;
  uint *puVar12;
  undefined *puVar13;
  int iVar14;
  short sVar15;
  uint uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 *local_74;
  undefined4 uStack_70;
  uint *local_6c;
  undefined4 local_68;
  undefined4 *local_64;
  HGDIOBJ pvStack_60;
  undefined4 *local_5c;
  undefined4 *local_58;
  undefined4 local_54;
  undefined4 local_50;
  uint local_4c;
  undefined4 auStack_48 [2];
  undefined4 local_40 [11];
  undefined4 uStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063253a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00601880(local_40,(char *)0x0,0,0,0,0,0,0);
  local_50 = *(uint **)((int)param_1 + 0x1c);
  local_4 = 0;
  local_68 = FUN_006037f0(7);
  puVar3 = (uint *)FUN_00451110(*(void **)((int)param_1 + 0x174),&local_54);
  local_4._0_1_ = 1;
  piVar4 = FUN_004f3220(*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c),puVar3);
  *(int **)((int)param_1 + 0x114) = piVar4;
  local_4._0_1_ = 0;
  FUN_00619730();
  iVar14 = *(int *)(*(int *)((int)param_1 + 0x118) + 0x9c);
  if (iVar14 == 1) {
    iVar14 = 1;
  }
  else if (iVar14 == 2) {
    iVar14 = 2;
  }
  else {
    iVar14 = 0;
  }
  iVar7 = *(int *)((int)param_1 + 0x174);
  *(undefined4 *)((int)param_1 + 0x188) = 1;
  if (*(int *)(iVar7 + 0x50) == 0) {
    uVar16 = 0x2d63;
    if (iVar14 == 1) {
      *(undefined4 *)((int)param_1 + 0x18c) = 0xd;
    }
    else {
      *(undefined4 *)((int)param_1 + 0x18c) = 0xf;
    }
  }
  else {
    uVar16 = local_4c;
    if (*(int *)(iVar7 + 0x50) == iVar14) {
      if (iVar14 == 1) {
        *(undefined4 *)((int)param_1 + 0x18c) = 0xc;
        uVar16 = (*(int *)(iVar7 + 0x54) != 2) + 0x2a06;
      }
      else if (iVar14 == 2) {
        *(undefined4 *)((int)param_1 + 0x18c) = 0xe;
        uVar16 = (-(uint)(*(int *)(iVar7 + 0x54) != 1) & 3) + 0x2a05;
      }
    }
    else if (*(int *)(iVar7 + 0x54) == iVar14) {
      if (iVar14 == 1) {
        *(undefined4 *)((int)param_1 + 0x18c) = 0xd;
        uVar16 = 0x2a05;
      }
      else if (iVar14 == 2) {
        *(undefined4 *)((int)param_1 + 0x18c) = 0xf;
        uVar16 = 0x2a06;
      }
    }
    else if (iVar14 == 1) {
      *(undefined4 *)((int)param_1 + 0x18c) = 0xd;
      uVar16 = 0x2a08;
    }
    else if (iVar14 == 2) {
      *(undefined4 *)((int)param_1 + 0x18c) = 0xf;
      uVar16 = 0x2a07;
    }
  }
  local_54 = (undefined4 *)FUN_00618b70(0x28);
  uVar17 = local_68;
  local_4._0_1_ = 2;
  if (local_54 == (undefined4 *)0x0) {
    local_74 = (undefined4 *)0x0;
  }
  else {
    local_74 = (undefined4 *)FUN_005fbd20(local_54,local_68,uVar16,10);
  }
  local_4._0_1_ = 0;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 0;
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    local_54 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 3;
    if (local_54 == (undefined4 *)0x0) {
      local_6c = (uint *)0x0;
    }
    else {
      local_6c = (uint *)FUN_005fbd20(local_54,uVar17,0x285f,10);
    }
    local_4._0_1_ = 0;
    local_54 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,4);
    if (local_54 != (undefined4 *)0x0) {
      local_64 = (undefined4 *)FUN_005fbd20(local_54,uVar17,0x2a44,10);
      goto LAB_0046f604;
    }
  }
  else {
    local_4._0_1_ = uVar2;
    local_54 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 5;
    if (local_54 == (undefined4 *)0x0) {
      local_6c = (uint *)0x0;
    }
    else {
      local_6c = (uint *)FUN_005fbd20(local_54,uVar17,0x2860,10);
    }
    local_4._0_1_ = 0;
    local_54 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,6);
    if (local_54 != (undefined4 *)0x0) {
      local_64 = (undefined4 *)FUN_005fbd20(local_54,uVar17,0x2a45,10);
      goto LAB_0046f604;
    }
  }
  local_64 = (undefined4 *)0x0;
LAB_0046f604:
  local_4 = local_4 & 0xffffff00;
  local_54 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 7;
  if (local_54 == (undefined4 *)0x0) {
    local_5c = (undefined4 *)0x0;
  }
  else {
    local_5c = (undefined4 *)FUN_005fbd20(local_54,uVar17,0x2a09,10);
  }
  local_4._0_1_ = 0;
  local_54 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 8;
  if (local_54 == (undefined4 *)0x0) {
    local_58 = (undefined4 *)0x0;
  }
  else {
    local_58 = (undefined4 *)FUN_005fbd20(local_54,uVar17,0x2a0a,10);
  }
  local_4._0_1_ = 0;
  local_54 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 9;
  if (local_54 == (undefined4 *)0x0) {
    local_54 = (undefined4 *)0x0;
  }
  else {
    local_54 = (undefined4 *)FUN_005fbd20(local_54,uVar17,0x2a0b,10);
  }
  local_4._0_1_ = 0;
  pHVar5 = GetDC(*(HWND *)((int)param_1 + 0x18));
  hdc = CreateCompatibleDC(pHVar5);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar5);
  iVar14 = 0xff;
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 2) {
    iVar14 = 0xff00;
  }
  pcVar6 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x174) + 0x88);
  FUN_00601aa0(local_40,pcVar6);
  FUN_00601c60(local_40,0xb);
  FUN_00403e90(local_40,1);
  FUN_00601b30(local_40,0xc,0x12);
  FUN_00601c90(local_40,iVar14,(HWND)0x0);
  this = (undefined4 *)((int)param_1 + 0x128);
  local_40[0] = 400;
  uStack_14 = 1;
  FUN_00601c60(this,0xb);
  FUN_00403e90(this,1);
  FUN_00601c90(this,iVar14,(HWND)0x0);
  *(undefined4 *)((int)param_1 + 0x154) = 1;
  pvStack_60 = (HGDIOBJ)FUN_00618b70(0x28);
  local_4._0_1_ = 10;
  if (pvStack_60 == (void *)0x0) {
    uStack_70 = (uint *)0x0;
  }
  else {
    puVar3 = local_6c;
    pHVar5 = hdc;
    iVar7 = FUN_005fc0f0((int *)local_6c);
    iVar8 = FUN_005fc0e0((int *)local_6c);
    uStack_70 = (uint *)FUN_005fbea0(pvStack_60,iVar8,iVar7,puVar3,pHVar5);
  }
  local_4._0_1_ = 0;
  FUN_005fcc30(local_74,uStack_70,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_6c,uStack_70,0,0);
  FUN_005fd0f0(local_64,uStack_70,0x19c,0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)uStack_70);
  pvStack_60 = SelectObject(hdc,pvVar9);
  FUN_00601ce0(local_40,hdc);
  FUN_00601b30(this,0xc,0xe1);
  *this = 400;
  pcVar6 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x174) + 0x94);
  FUN_00601aa0(this,pcVar6);
  FUN_00601ce0(this,hdc);
  FUN_00601c60(this,6);
  FUN_00403e90(this,0x110);
  *(undefined4 *)((int)param_1 + 0x154) = 1;
  *this = 0x160;
  FUN_00601b30(this,0x19,0x100);
  FUN_00601c90(this,0,(HWND)0x0);
  FUN_00601b80(this,hdc);
  FUN_00601ce0(this,hdc);
  pcVar6 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x174) + 0xa0);
  FUN_00601aa0(this,pcVar6);
  FUN_00601c90(this,iVar14,(HWND)0x0);
  FUN_00601b30(this,0x18,0xff);
  FUN_00601b80(this,hdc);
  FUN_00601ce0(this,hdc);
  FUN_006075b0(param_1,(int *)uStack_70,1);
  uStack_70 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xb;
  if (uStack_70 == (uint *)0x0) {
    uStack_70 = (uint *)0x0;
  }
  else {
    puVar3 = local_6c;
    pHVar5 = hdc;
    iVar7 = FUN_005fc0f0((int *)local_6c);
    iVar8 = FUN_005fc0e0((int *)local_6c);
    uStack_70 = (uint *)FUN_005fbea0(uStack_70,iVar8,iVar7,puVar3,pHVar5);
  }
  local_4._0_1_ = 0;
  FUN_005fcc30(local_54,uStack_70,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_6c,uStack_70,0,0);
  FUN_005fd0f0(local_64,uStack_70,0x19c,0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)uStack_70);
  SelectObject(hdc,pvVar9);
  FUN_00601ce0(local_40,hdc);
  FUN_00601c60(this,6);
  FUN_00403e90(this,0x110);
  FUN_00601c90(this,iVar14,(HWND)0x0);
  *(undefined4 *)((int)param_1 + 0x154) = 1;
  *this = 0x148;
  FUN_00601b30(this,0x18,0x28);
  pcVar6 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x174) + 0xac);
  FUN_00601aa0(this,pcVar6);
  FUN_00601ce0(this,hdc);
  FUN_00601c60(this,7);
  FUN_00403e90(this,0x110);
  FUN_00601c90(this,iVar14,(HWND)0x0);
  *(undefined4 *)((int)param_1 + 0x154) = 1;
  *this = 0x148;
  FUN_00601b30(this,0x18,0x82);
  pcVar6 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x174) + 0xb8);
  FUN_00601aa0(this,pcVar6);
  FUN_00601ce0(this,hdc);
  FUN_00607610(param_1,(int *)uStack_70,4);
  FUN_00601b30(this,0x18,0xff);
  FUN_00601c60(this,4);
  FUN_00601c90(this,iVar14,(HWND)0x0);
  FUN_00403e90(this,1);
  *(undefined4 *)((int)param_1 + 0x154) = 1;
  pcVar6 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x174) + 0x88);
  FUN_00601aa0(local_40,pcVar6);
  pvVar10 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xc;
  uStack_70 = (uint *)pvVar10;
  if (pvVar10 == (void *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = local_6c;
    pHVar5 = hdc;
    iVar14 = FUN_005fc0f0((int *)local_6c);
    iVar7 = FUN_005fc0e0((int *)local_6c);
    puVar3 = (uint *)FUN_005fbea0(pvVar10,iVar7,iVar14,puVar3,pHVar5);
  }
  local_4._0_1_ = 0;
  FUN_005fcc30(local_5c,puVar3,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_6c,puVar3,0,0);
  FUN_005fd0f0(local_64,puVar3,0x19c,0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)puVar3);
  SelectObject(hdc,pvVar9);
  FUN_00601ce0(local_40,hdc);
  FUN_00601c90(this,0xffffff,(HWND)0x0);
  *this = 0xa5;
  FUN_00601b30(this,0x25,0x76);
  uStack_70 = (uint *)CONCAT22(DAT_0065d424,0x1932);
  FUN_006019a0(this,&uStack_70);
  FUN_00601ce0(this,hdc);
  FUN_00601b30(this,0xcc,0x76);
  uStack_70 = (uint *)CONCAT22(DAT_0065d424,0x1933);
  FUN_006019a0(this,&uStack_70);
  FUN_00601ce0(this,hdc);
  FUN_00601b30(this,0x56,0x46);
  FUN_00403e90(this,0);
  uStack_70 = (uint *)CONCAT22(DAT_0065d424,0x1952);
  FUN_006019a0(this,&uStack_70);
  FUN_00601ce0(this,hdc);
  FUN_00607610(param_1,(int *)puVar3,2);
  FUN_00403e90(this,1);
  SelectObject(hdc,pvStack_60);
  pvVar10 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xd;
  pvStack_60 = pvVar10;
  if (pvVar10 == (void *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = local_6c;
    pHVar5 = hdc;
    iVar14 = FUN_005fc0f0((int *)local_6c);
    iVar7 = FUN_005fc0e0((int *)local_6c);
    puVar3 = (uint *)FUN_005fbea0(pvVar10,iVar7,iVar14,puVar3,pHVar5);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005fcc30(local_58,puVar3,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_6c,puVar3,0,0);
  FUN_005fd0f0(local_64,puVar3,0x19c,0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)puVar3);
  pvStack_60 = SelectObject(hdc,pvVar9);
  FUN_00601ce0(local_40,hdc);
  *this = 0x6e;
  uStack_70 = (uint *)CONCAT22(DAT_0065d424,0x1934);
  FUN_006019a0(this,&uStack_70);
  FUN_00601b30(this,0x25,0x76);
  FUN_00601b80(this,hdc);
  FUN_00601ce0(this,hdc);
  uStack_70 = (uint *)CONCAT22(DAT_0065d424,0x1935);
  FUN_006019a0(this,&uStack_70);
  FUN_00601b30(this,0x95,0x76);
  FUN_00601ce0(this,hdc);
  uStack_70 = (uint *)CONCAT22(DAT_0065d424,0x1936);
  FUN_006019a0(this,&uStack_70);
  FUN_00601b30(this,0x104,0x76);
  FUN_00601ce0(this,hdc);
  FUN_00601b30(this,0x56,0x46);
  FUN_00403e90(this,0);
  uStack_70 = (uint *)CONCAT22(DAT_0065d424,0x1952);
  FUN_006019a0(this,&uStack_70);
  FUN_00601ce0(this,hdc);
  FUN_00607610(param_1,(int *)puVar3,3);
  SelectObject(hdc,pvStack_60);
  puVar3 = local_6c;
  if (local_6c != (uint *)0x0) {
    FUN_005fbfa0(local_6c);
    FUN_00618b60((undefined *)puVar3);
  }
  if (local_74 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_74);
    FUN_00618b60((undefined *)local_74);
  }
  puVar11 = local_5c;
  if (local_5c != (undefined4 *)0x0) {
    FUN_005fbfa0(local_5c);
    FUN_00618b60((undefined *)puVar11);
  }
  puVar11 = local_58;
  if (local_58 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_58);
    FUN_00618b60((undefined *)puVar11);
  }
  puVar11 = local_54;
  if (local_54 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_54);
    FUN_00618b60((undefined *)puVar11);
  }
  puVar11 = local_64;
  if (local_64 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_64);
    FUN_00618b60((undefined *)puVar11);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    sVar15 = 0x1a2;
    pvStack_60 = (HGDIOBJ)0x58;
    uStack_70 = (uint *)0x29;
    local_64 = (undefined4 *)0xcb;
  }
  else {
    sVar15 = 0x1aa;
    pvStack_60 = (HGDIOBJ)0x59;
    uStack_70 = (uint *)0x2c;
    local_64 = (undefined4 *)0xda;
  }
  local_54 = (undefined4 *)FUN_00618b70(0x9c);
  local_4._0_1_ = 0xe;
  if (local_54 == (undefined4 *)0x0) {
    pvVar10 = (void *)0x0;
  }
  else {
    puVar3 = (uint *)FUN_006073d0(param_1);
    pvVar10 = FUN_0060d590(local_54,local_50,(int)sVar15,(int)(short)pvStack_60,
                           (int)(short)uStack_70,(int)(short)local_64,0x6e,param_1,puVar3);
  }
  local_4._0_1_ = 0;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 0;
  *(void **)((int)param_1 + 0x15c) = pvVar10;
  if (pvVar10 != (void *)0x0) {
    FUN_005ffce0(pvVar10,0);
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uStack_70 = (uint *)0x29;
      uVar16 = 0x29e8;
      local_74 = (undefined4 *)0x29e9;
    }
    else {
      uStack_70 = (uint *)0x2c;
      uVar16 = 0x29f2;
      local_74 = (undefined4 *)0x29f3;
    }
    local_54 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0xf;
    if (local_54 == (undefined4 *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_54,local_68,0,0,(int)(short)uStack_70,0x29,
                                   *(int *)((int)param_1 + 0x15c),0x6f,uVar16,(uint)local_74,
                                   *(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0,0);
    }
    local_4._0_1_ = 0;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      FUN_00602840(piVar4,0x260);
      local_54 = (undefined4 *)CONCAT22(DAT_0065d424,0x1922);
      FUN_00600a40(param_1,piVar4,&local_54);
      FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar4,1,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      sVar15 = 0x36;
      local_6c = (uint *)0x29ea;
      local_74 = (undefined4 *)0x29eb;
    }
    else {
      sVar15 = 0x3b;
      local_6c = (uint *)0x29f4;
      local_74 = (undefined4 *)0x29f5;
    }
    local_54 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x10;
    if (local_54 == (undefined4 *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_54,local_68,0,(int)sVar15,(int)(short)uStack_70,0x29,
                                   *(int *)((int)param_1 + 0x15c),0x70,(uint)local_6c,(uint)local_74
                                   ,*(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0,0);
    }
    local_4._0_1_ = 0;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      FUN_00602840(piVar4,0x260);
      local_54 = (undefined4 *)CONCAT22(DAT_0065d424,0x1937);
      FUN_00600a40(param_1,piVar4,&local_54);
      FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar4,0,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      sVar15 = 0x6c;
      local_6c = (uint *)0x29ec;
      local_74 = (undefined4 *)0x29ed;
    }
    else {
      sVar15 = 0x76;
      local_6c = (uint *)0x29f6;
      local_74 = (undefined4 *)0x29f7;
    }
    local_54 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x11;
    if (local_54 == (undefined4 *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_54,local_68,0,(int)sVar15,(int)(short)uStack_70,0x29,
                                   *(int *)((int)param_1 + 0x15c),0x71,(uint)local_6c,(uint)local_74
                                   ,*(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0,0);
    }
    local_4._0_1_ = 0;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      FUN_00602840(piVar4,0x260);
      local_54 = (undefined4 *)CONCAT22(DAT_0065d424,0x1938);
      FUN_00600a40(param_1,piVar4,&local_54);
      FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar4,0,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      sVar15 = 0xa2;
      local_6c = (uint *)0x2a27;
      local_74 = (undefined4 *)0x2a28;
    }
    else {
      sVar15 = 0xb1;
      local_6c = (uint *)0x2918;
      local_74 = (undefined4 *)0x2919;
    }
    local_54 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x12;
    if (local_54 == (undefined4 *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_54,local_68,0,(int)sVar15,(int)(short)uStack_70,0x29,
                                   *(int *)((int)param_1 + 0x15c),0x72,(uint)local_6c,(uint)local_74
                                   ,*(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0,0);
    }
    local_4._0_1_ = 0;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      FUN_00602840(piVar4,0x260);
      local_54 = (undefined4 *)CONCAT22(DAT_0065d424,0x1939);
      FUN_00600a40(param_1,piVar4,&local_54);
      FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar4,0,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      sVar15 = 0xa2;
      local_6c = (uint *)0x2916;
      local_74 = (undefined4 *)0x2917;
    }
    else {
      sVar15 = 0xb1;
      local_6c = (uint *)0x2918;
      local_74 = (undefined4 *)0x2919;
    }
    local_54 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x13;
    if (local_54 == (undefined4 *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_54,local_68,0,(int)sVar15,(int)(short)uStack_70,0x29,
                                   *(int *)((int)param_1 + 0x15c),0x72,(uint)local_6c,(uint)local_74
                                   ,*(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0,0);
    }
    local_4._0_1_ = 0;
    uVar2 = (undefined1)local_4;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      FUN_00602840(piVar4,0x260);
      local_54 = (undefined4 *)CONCAT22(DAT_0065d424,0x1931);
      FUN_00600a40(param_1,piVar4,&local_54);
      FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar4,0,0);
      uVar2 = (undefined1)local_4;
    }
  }
  local_4._0_1_ = uVar2;
  local_54 = (undefined4 *)FUN_00618b70(0x10);
  local_4._0_1_ = 0x14;
  if (local_54 == (undefined4 *)0x0) {
    pvVar10 = (void *)0x0;
  }
  else {
    pvVar10 = FUN_0060a790(local_54,4);
  }
  local_4._0_1_ = 0;
  *(void **)((int)param_1 + 0x124) = pvVar10;
  pvVar10 = (void *)FUN_00618b70(0x118);
  local_4._0_1_ = 0x15;
  local_54 = (undefined4 *)pvVar10;
  if (pvVar10 == (void *)0x0) {
    pvVar10 = (void *)0x0;
  }
  else {
    uVar19 = 0x46;
    uVar18 = 0xa5;
    uVar17 = 200;
    puVar3 = (uint *)FUN_006073d0(param_1);
    pvVar10 = FUN_00607ea0(pvVar10,local_50,0x25,0x88,0x15b,0xa6,param_1,puVar3,uVar17,uVar18,uVar19
                          );
  }
  local_4._0_1_ = 0;
  *(void **)((int)param_1 + 0x120) = pvVar10;
  if (pvVar10 != (void *)0x0) {
    *(undefined4 *)((int)pvVar10 + 0xf4) = 0xb0000;
    *(uint *)(*(int *)((int)param_1 + 0x120) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 0x120) + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x120),0);
    iVar14 = *(int *)((int)param_1 + 0x120);
    FUN_0060a490((void *)(iVar14 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar14,7,0x299d,
                 *(undefined4 *)(iVar14 + 0x3c),0,*(undefined4 *)(iVar14 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x120) + 0xe4) = 4;
    FUN_00608280(*(void **)((int)param_1 + 0x120),*(undefined4 *)((int)param_1 + 0x124));
    FUN_00609950(*(void **)((int)param_1 + 0x120),0xffffff);
  }
  pvVar10 = (void *)FUN_00618b70(0x9c);
  local_4._0_1_ = 0x16;
  local_54 = (undefined4 *)pvVar10;
  if (pvVar10 == (void *)0x0) {
    pvVar10 = (void *)0x0;
  }
  else {
    puVar3 = (uint *)FUN_006076b0(param_1,3);
    pvVar10 = FUN_0060d590(pvVar10,local_50,0x82,0x3b,0xc4,0x2a,100,param_1,puVar3);
  }
  local_4._0_1_ = 0;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 0;
  *(void **)((int)param_1 + 0x160) = pvVar10;
  if (pvVar10 != (void *)0x0) {
    *(uint *)((int)pvVar10 + 0x50) = *(uint *)((int)pvVar10 + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x160),0);
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uStack_70 = (uint *)0x31;
      local_64 = (undefined4 *)0x2a;
      uVar16 = 0x286c;
      local_74 = (undefined4 *)0x286b;
    }
    else {
      uStack_70 = (uint *)0x2c;
      local_64 = (undefined4 *)0x29;
      uVar16 = 0x2878;
      local_74 = (undefined4 *)0x2877;
    }
    local_50 = (uint *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x17;
    if (local_50 == (uint *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_50,local_68,0,0,(int)(short)uStack_70,(int)(short)local_64,
                                   *(int *)((int)param_1 + 0x160),0x65,uVar16,(uint)local_74,
                                   *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4._0_1_ = 0;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      local_50 = (uint *)CONCAT22(DAT_0065d424,0x1928);
      FUN_00600a40(param_1,piVar4,&local_50);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar4,1,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uVar16 = 0x2872;
      local_74 = (undefined4 *)0x2871;
    }
    else {
      uVar16 = 0x287c;
      local_74 = (undefined4 *)0x287b;
    }
    local_50 = (uint *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x18;
    if (local_50 == (uint *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_50,local_68,0x31,0,(int)(short)uStack_70,
                                   (int)(short)local_64,*(int *)((int)param_1 + 0x160),0x66,uVar16,
                                   (uint)local_74,*(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),
                                   0,0);
    }
    local_4._0_1_ = 0;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      local_50 = (uint *)CONCAT22(DAT_0065d424,0x1929);
      FUN_00600a40(param_1,piVar4,&local_50);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar4,0,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uVar16 = 0x2870;
      local_74 = (undefined4 *)0x286f;
    }
    else {
      uVar16 = 0x287a;
      local_74 = (undefined4 *)0x2879;
    }
    local_50 = (uint *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x19;
    if (local_50 == (uint *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_50,local_68,0x62,0,(int)(short)uStack_70,
                                   (int)(short)local_64,*(int *)((int)param_1 + 0x160),0x67,uVar16,
                                   (uint)local_74,*(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),
                                   0,0);
    }
    local_4._0_1_ = 0;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      local_50 = (uint *)CONCAT22(DAT_0065d424,0x1930);
      FUN_00600a40(param_1,piVar4,&local_50);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar4,0,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uVar16 = 0x286a;
      local_74 = (undefined4 *)0x2869;
    }
    else {
      uVar16 = 0x2876;
      local_74 = (undefined4 *)0x2875;
    }
    local_50 = (uint *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x1a;
    if (local_50 == (uint *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)FUN_00602150(local_50,local_68,0x93,0,(int)(short)uStack_70,
                                   (int)(short)local_64,*(int *)((int)param_1 + 0x160),0x68,uVar16,
                                   (uint)local_74,*(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),
                                   0,0);
    }
    local_4._0_1_ = 0;
    uVar2 = (undefined1)local_4;
    if (piVar4 != (int *)0x0) {
      FUN_005ffce0(piVar4,0);
      local_50 = (uint *)CONCAT22(DAT_0065d424,0x1931);
      FUN_00600a40(param_1,piVar4,&local_50);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar4,0,0);
      uVar2 = (undefined1)local_4;
    }
  }
  local_4._0_1_ = uVar2;
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    sVar15 = 0x1a7;
    pvStack_60 = (HGDIOBJ)0x19;
    uStack_70 = (uint *)0x20;
    local_64 = (undefined4 *)0x1f;
    local_6c = (uint *)0x2882;
    local_74 = (undefined4 *)0x2883;
  }
  else {
    sVar15 = 0x1aa;
    pvStack_60 = (HGDIOBJ)0x11;
    uStack_70 = (uint *)0x2c;
    local_64 = (undefined4 *)0x29;
    local_6c = (uint *)0x2888;
    local_74 = (undefined4 *)0x2889;
  }
  local_50 = (uint *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x1b;
  if (local_50 == (uint *)0x0) {
    pvVar10 = (void *)0x0;
  }
  else {
    iVar7 = 0;
    iVar14 = 0;
    puVar3 = (uint *)FUN_006073d0(param_1);
    pvVar10 = FUN_00602150(local_50,local_68,(int)sVar15,(int)(short)pvStack_60,
                           (int)(short)uStack_70,(int)(short)local_64,param_1,0xc9,(uint)local_6c,
                           (uint)local_74,puVar3,iVar14,iVar7);
  }
  local_4._0_1_ = 0;
  *(void **)((int)param_1 + 0x164) = pvVar10;
  FUN_005ffce0(pvVar10,0);
  FUN_00602840(*(void **)((int)param_1 + 0x164),0x260);
  FUN_004f5510(auStack_48);
  local_4._0_1_ = 0x1c;
  bVar1 = false;
  FUN_004727c0(*(void **)((int)param_1 + 0x174),&local_4c);
  local_4 = CONCAT31(local_4._1_3_,0x1d);
  iVar14 = FUN_004ece60(&local_4c);
  if ((iVar14 != 0) &&
     (piVar4 = FUN_004f36a0(*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c),&local_4c),
     piVar4 != (int *)0x0)) {
    bVar1 = true;
    *(int *)((int)param_1 + 0x170) = piVar4[7];
  }
  if (bVar1) {
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      local_6c = (uint *)0x2a0d;
      local_74 = (undefined4 *)0x2a0f;
    }
    else {
      local_6c = (uint *)0x2a0e;
      local_74 = (undefined4 *)0x2a10;
    }
    pvVar10 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x1e;
    local_50 = (uint *)pvVar10;
    if (pvVar10 == (void *)0x0) {
      pvVar10 = (void *)0x0;
    }
    else {
      iVar7 = 1;
      iVar14 = 0;
      puVar3 = (uint *)FUN_006076b0(param_1,4);
      pvVar10 = FUN_00602150(pvVar10,local_68,0xe5,0xb6,0xa9,0x60,param_1,0xca,(uint)local_6c,
                             (uint)local_74,puVar3,iVar14,iVar7);
    }
    local_4._0_1_ = 0x1d;
    *(void **)((int)param_1 + 0x168) = pvVar10;
    local_50 = (uint *)FUN_00603550(pvVar10,0);
    local_54 = (undefined4 *)FUN_00603550(*(void **)((int)param_1 + 0x168),1);
    puVar11 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x1f;
    local_58 = puVar11;
    if (puVar11 == (undefined4 *)0x0) {
      puVar3 = (uint *)0x0;
    }
    else {
      puVar3 = local_50;
      iVar14 = FUN_005fc0f0((int *)local_50);
      iVar7 = FUN_005fc0e0((int *)local_50);
      puVar3 = (uint *)FUN_005fbda0(puVar11,iVar7,iVar14,puVar3);
    }
    local_4 = CONCAT31(local_4._1_3_,0x1d);
    FUN_005fcc30(local_50,puVar3,0,0,0,0,(undefined4 *)0x0,0);
    FUN_005fd0f0(local_54,puVar3,0,0);
    FUN_00603350(*(void **)((int)param_1 + 0x168),1,(int)puVar3);
    *(uint *)(*(int *)((int)param_1 + 0x168) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 0x168) + 0x50) & 0xefffffff;
    FUN_006027e0(*(void **)((int)param_1 + 0x168),0,0x60);
    FUN_005ffce0(*(void **)((int)param_1 + 0x168),0);
    iVar14 = FUN_006076b0(param_1,4);
    pvVar9 = (HGDIOBJ)FUN_005fc130(iVar14);
    pvStack_60 = SelectObject(hdc,pvVar9);
    local_50 = (uint *)CONCAT22(DAT_0065d424,0x1942);
    FUN_006019a0(this,&local_50);
    FUN_00601c60(this,4);
    FUN_00601c90(this,0xffffff,(HWND)0x0);
    FUN_00403e90(this,0x101);
    *(undefined4 *)((int)param_1 + 0x154) = 1;
    *this = 0xa9;
    FUN_00601b30(this,0xe5,0x116);
    FUN_00601ce0(this,hdc);
    SelectObject(hdc,pvStack_60);
  }
  iVar14 = *(int *)(*(int *)((int)param_1 + 0x118) + 0x9c);
  puVar3 = (uint *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x20;
  local_50 = puVar3;
  if (puVar3 == (uint *)0x0) {
    pvVar10 = (void *)0x0;
  }
  else {
    iVar8 = 1;
    iVar7 = 0;
    puVar12 = (uint *)FUN_006076b0(param_1,4);
    pvVar10 = FUN_00602150(puVar3,local_68,0x1d,0xb6,0xa9,0x60,param_1,0xcb,0x2a0c,
                           (iVar14 != 1) + 0x2a0f,puVar12,iVar7,iVar8);
  }
  local_4._0_1_ = 0x1d;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 0x1d;
  *(void **)((int)param_1 + 0x16c) = pvVar10;
  if (pvVar10 != (void *)0x0) {
    local_50 = (uint *)FUN_00603550(pvVar10,0);
    local_54 = (undefined4 *)FUN_00603550(*(void **)((int)param_1 + 0x16c),1);
    puVar11 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x21;
    local_58 = puVar11;
    if (puVar11 == (undefined4 *)0x0) {
      puVar3 = (uint *)0x0;
    }
    else {
      puVar3 = local_50;
      iVar14 = FUN_005fc0f0((int *)local_50);
      iVar7 = FUN_005fc0e0((int *)local_50);
      puVar3 = (uint *)FUN_005fbda0(puVar11,iVar7,iVar14,puVar3);
    }
    local_4._0_1_ = 0x1d;
    FUN_005fcc30(local_50,puVar3,0,0,0,0,(undefined4 *)0x0,0);
    FUN_005fd0f0(local_54,puVar3,0,0);
    FUN_00603350(*(void **)((int)param_1 + 0x16c),1,(int)puVar3);
    *(uint *)(*(int *)((int)param_1 + 0x16c) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 0x16c) + 0x50) & 0xefffffff;
    FUN_006027e0(*(void **)((int)param_1 + 0x16c),0,0x60);
    FUN_005ffce0(*(void **)((int)param_1 + 0x16c),0);
    iVar14 = FUN_006076b0(param_1,4);
    pvVar9 = (HGDIOBJ)FUN_005fc130(iVar14);
    pvVar9 = SelectObject(hdc,pvVar9);
    if (*(int *)((int)param_1 + 0x114) != 0) {
      puVar13 = FUN_004f62d0(*(int *)((int)param_1 + 0x114));
      pcVar6 = (char *)FUN_00583c40((int)puVar13);
      FUN_00601aa0(this,pcVar6);
    }
    FUN_00601c60(this,4);
    FUN_00601c90(this,0xffffff,(HWND)0x0);
    FUN_00403e90(this,0x101);
    *(undefined4 *)((int)param_1 + 0x154) = 1;
    *this = 0xa9;
    FUN_00601b30(this,0x1d,0x116);
    FUN_00601ce0(this,hdc);
    SelectObject(hdc,pvVar9);
    uVar2 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar2;
  DeleteDC(hdc);
  iVar14 = *(int *)((int)param_1 + 0x118);
  FUN_00606980(param_1,*(int *)(iVar14 + 0xcc),*(int *)(iVar14 + 0xd0),*(int *)(iVar14 + 0xd4),
               *(int *)(iVar14 + 0xd8));
  FUN_00471340(param_1,0x6f);
  local_4._0_1_ = 0x1c;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f55c0(auStack_48);
  local_4 = 0xffffffff;
  FUN_00601990((int)local_40);
  ExceptionList = pvStack_c;
  return;
}

