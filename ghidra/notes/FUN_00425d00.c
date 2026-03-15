
/* WARNING: Type propagation algorithm not settling */

void __thiscall FUN_00425d00(void *this,int param_1,int param_2)

{
  int iVar1;
  int iVar2;
  void *pvVar3;
  BOOL BVar4;
  int *piVar5;
  uint *puVar6;
  char *pcVar7;
  HGDIOBJ pvVar8;
  undefined4 *puVar9;
  uint yTop;
  longlong lVar10;
  longlong lVar11;
  undefined4 uStack_f4;
  RECT RStack_f0;
  undefined4 local_e0;
  uint *local_dc;
  undefined4 local_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 local_bc;
  undefined4 local_b8;
  undefined2 uStack_b4;
  undefined2 uStack_b2;
  undefined2 uStack_b0;
  undefined2 uStack_ae;
  undefined2 uStack_ac;
  undefined2 uStack_aa;
  undefined2 uStack_a8;
  undefined2 uStack_a6;
  undefined2 uStack_a4;
  undefined2 uStack_a2;
  undefined2 uStack_a0;
  undefined2 uStack_9e;
  undefined2 uStack_9c;
  undefined2 uStack_9a;
  undefined2 uStack_98;
  undefined2 uStack_96;
  undefined2 uStack_94;
  undefined2 uStack_92;
  undefined2 uStack_90;
  undefined2 uStack_8e;
  undefined2 uStack_8c;
  undefined2 uStack_8a;
  undefined2 uStack_88;
  undefined2 uStack_86;
  undefined2 uStack_84;
  undefined2 uStack_82;
  undefined2 uStack_80;
  undefined2 uStack_7e;
  undefined2 uStack_7c;
  undefined2 uStack_7a;
  undefined2 uStack_78;
  undefined2 uStack_76;
  undefined2 uStack_74;
  undefined2 uStack_72;
  undefined2 uStack_70;
  undefined2 uStack_6e;
  undefined2 uStack_6c;
  undefined2 uStack_6a;
  int aiStack_68 [5];
  void *pvStack_54;
  LONG LStack_50;
  int iStack_4c;
  LONG LStack_48;
  undefined4 uStack_3c;
  tagRECT tStack_34;
  undefined1 local_24 [4];
  undefined4 uStack_20;
  undefined4 uStack_1c;
  uint uStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c0a9;
  pvStack_c = ExceptionList;
  puVar9 = (undefined4 *)0x0;
  local_e0 = 0;
  local_b8 = (undefined4 *)0x0;
  ExceptionList = &pvStack_c;
  local_d8 = this;
  FUN_00610b20(local_24,0,0,0,0,0,0);
  local_4 = 0;
  local_bc = 0;
  if (*(short *)((int)this + 0x494) != 0) {
    local_bc = *(undefined4 *)((int)this + 0x494);
  }
  local_dc = (uint *)0x0;
  *(undefined4 *)((int)this + 0x494) = 0;
  if (*(int **)((int)this + 0x118) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x118) + 8))();
  }
  (**(code **)(*(int *)((int)this + 0x2cc) + 4))();
  if ((param_1 != 0x92) && (param_1 != 0x93)) {
    param_2 = 0;
  }
  if (param_2 != 0) {
    for (iVar2 = thunk_FUN_005f5060(param_2); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
      uStack_f4 = (void *)FUN_00618b70(0x1c);
      local_4._0_1_ = 1;
      if (uStack_f4 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_005f55d0(uStack_f4,*(undefined4 *)(iVar2 + 0x18));
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f5440((void *)((int)this + 0x2cc),pvVar3);
    }
  }
  if ((*(int *)((int)this + 200) == param_1) && (param_2 == 0)) {
    ExceptionList = pvStack_c;
    return;
  }
  *(int *)((int)this + 200) = param_1;
  if (param_1 == 0x80) {
    FUN_00427010(this,(uint *)0x0);
    piVar5 = *(int **)((int)this + 0x124);
    if (piVar5 == (int *)0x0) {
      ExceptionList = pvStack_c;
      return;
    }
    *(int *)((int)this + 0x11c) = piVar5[10];
    *(int *)((int)this + 0x120) = piVar5[0xb];
    (**(code **)(*piVar5 + 8))();
    FUN_00600280(*(int *)((int)this + 0x124));
    if (*(undefined4 **)((int)this + 0x124) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0x124))(1);
    }
    *(undefined4 *)((int)this + 0x124) = 0;
    ExceptionList = pvStack_c;
    return;
  }
  if (*(int *)((int)this + 0x124) != 0) {
    BVar4 = IsWindowVisible(*(HWND *)(*(int *)((int)this + 0x124) + 0x18));
    piVar5 = *(int **)((int)this + 0x124);
    if (BVar4 == 0) {
      FUN_00600280((int)piVar5);
      if (*(undefined4 **)((int)this + 0x124) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)this + 0x124))(1);
      }
    }
    else {
      *(int *)((int)this + 0x11c) = piVar5[10];
      *(int *)((int)this + 0x120) = piVar5[0xb];
      (**(code **)(*piVar5 + 8))();
      FUN_00600280(*(int *)((int)this + 0x124));
      if (*(undefined4 **)((int)this + 0x124) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)this + 0x124))(1);
      }
    }
    *(undefined4 *)((int)this + 0x124) = 0;
  }
  local_dc = (uint *)FUN_00427010(this,(uint *)0x1);
  uStack_c4 = (void *)((int)this + 0x484);
  *(int *)((int)this + 200) = param_1;
  FUN_005f5b20(uStack_c4);
  piVar5 = FUN_004f3900(*(int *)((int)this + 0x9c));
  while (uStack_d4 = piVar5, piVar5 != (int *)0x0) {
    FUN_00512f90(aiStack_68,piVar5,4);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00513120((int)aiStack_68);
    while (iStack_4c != 0) {
      uStack_d0 = (int *)FUN_0052bed0((int)aiStack_68);
      FUN_00509620(uStack_d0,&uStack_cc);
      uStack_f4 = (void *)(DAT_00658bd8 + 1);
      lVar10 = __ftol();
      uStack_f4 = (void *)(DAT_00658bda + 1);
      lVar11 = __ftol();
      if (*(int *)((int)this + 0x9c) == 1) {
        iVar2 = (int)lVar10 + 0x15;
        yTop = (int)lVar11 + 0x19;
      }
      else {
        iVar2 = (int)lVar10 + 0x54;
        yTop = (int)lVar11 + 0x1b;
      }
      pvVar3 = (void *)FUN_00618b70(0x74);
      local_4._0_1_ = 4;
      uStack_f4 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        puVar6 = FUN_004025b0(uStack_d0,&uStack_c0);
        local_e0 = local_e0 | 1;
        local_4 = CONCAT31(local_4._1_3_,5);
        pvVar3 = FUN_004421d0(pvVar3,puVar6);
      }
      local_4 = 4;
      if ((local_e0 & 1) != 0) {
        local_e0 = local_e0 & 0xfffffffe;
        FUN_00619730();
      }
      local_4._0_1_ = 3;
      *(int *)((int)pvVar3 + 0x28) = iVar2;
      *(uint *)((int)pvVar3 + 0x2c) = yTop;
      SetRect(&tStack_34,iVar2,yTop,iVar2 + 0x11,yTop + 0x11);
      *(LONG *)((int)pvVar3 + 0x40) = tStack_34.left;
      *(LONG *)((int)pvVar3 + 0x44) = tStack_34.top;
      *(LONG *)((int)pvVar3 + 0x48) = tStack_34.right;
      *(LONG *)((int)pvVar3 + 0x4c) = tStack_34.bottom;
      uStack_f4 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 6;
      if (uStack_f4 == (void *)0x0) {
        puVar6 = (uint *)0x0;
      }
      else {
        puVar6 = (uint *)FUN_005fbda0(uStack_f4,0x11,0x11,local_dc);
      }
      local_4._0_1_ = 3;
      FUN_005fcc30(local_dc,puVar6,0,0,iVar2,yTop,(undefined4 *)0x0,0);
      FUN_0060be60(pvVar3,0,(int)puVar6);
      FUN_005f59f0(uStack_c4,(int)pvVar3);
      piVar5 = uStack_d0;
      puVar6 = FUN_004025b0(uStack_d0,&uStack_c8);
      local_4._0_1_ = 7;
      FUN_0042b330(local_d8,puVar6,piVar5,0);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      FUN_005130d0((int)aiStack_68);
      puVar9 = local_b8;
      this = local_d8;
      piVar5 = uStack_d4;
    }
    FUN_0042cde0(this,(uint)piVar5);
    local_4 = local_4 & 0xffffff00;
    FUN_00513010(aiStack_68);
    RStack_f0.left = 0x80;
    RStack_f0.top = 0x90;
    local_4._0_1_ = 2;
    piVar5 = (int *)FUN_004f6010(piVar5,(uint *)&RStack_f0,1);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  if ((short)local_bc == 0) {
    iVar2 = *(int *)((int)this + 200);
    switch(iVar2) {
    case 0x10:
    case 0x11:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,9);
      if ((void *)RStack_f0.left == (void *)0x0) {
LAB_00426a17:
        puVar9 = (undefined4 *)0x0;
      }
      else {
        uStack_d4 = (int *)CONCAT22(DAT_0065d424,0x1615);
        puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_d4);
      }
      break;
    case 0x12:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,10);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_d0 = (int *)CONCAT22(DAT_0065d424,0x1616);
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_d0);
      break;
    case 0x13:
    case 0x15:
      if ((*(int *)((int)this + 0x9c) == 1) == (iVar2 == 0x13)) {
        RStack_f0.left = FUN_00618b70(0xc);
        local_4 = CONCAT31(local_4._1_3_,0xb);
        if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
        uStack_cc._0_2_ = 0x161f;
        uStack_cc._2_2_ = DAT_0065d424;
        puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_cc);
      }
      else {
        RStack_f0.left = FUN_00618b70(0xc);
        local_4 = CONCAT31(local_4._1_3_,0xc);
        if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
        local_d8 = (void *)CONCAT22(DAT_0065d424,0x1620);
        puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&local_d8);
      }
      break;
    case 0x14:
    case 0x16:
      if ((iVar2 == 0x14) != (*(int *)((int)this + 0x9c) == 2)) {
        puVar9 = (undefined4 *)0x0;
        goto switchD_0042622f_caseD_18;
      }
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0xd);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      local_e0 = CONCAT22(DAT_0065d424,0x1621);
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&local_e0);
      break;
    case 0x17:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0xe);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_70 = 0x1629;
      uStack_6e = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_70);
      break;
    default:
      goto switchD_0042622f_caseD_18;
    case 0x20:
    case 0x21:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0xf);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_9e = DAT_0065d424;
      uStack_a0 = 0x1617;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_a0);
      break;
    case 0x22:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x10);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_98 = 0x1618;
      uStack_96 = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_98);
      break;
    case 0x41:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x11);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_8e = DAT_0065d424;
      uStack_90 = 0x1672;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_90);
      break;
    case 0x42:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x12);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_88 = 0x1669;
      uStack_86 = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_88);
      break;
    case 0x43:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x13);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_7e = DAT_0065d424;
      uStack_80 = 0x161d;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_80);
      break;
    case 0x44:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x14);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_78 = 0x161e;
      uStack_76 = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_78);
      break;
    case 0x51:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x15);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_aa = DAT_0065d424;
      uStack_ac = 0x1606;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_ac);
      break;
    case 0x52:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x16);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_84 = 0x1607;
      uStack_82 = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_84);
      break;
    case 0x53:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x17);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_a2 = DAT_0065d424;
      uStack_a4 = 0x1608;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_a4);
      break;
    case 0x54:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x18);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_b4 = 0x1609;
      uStack_b2 = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_b4);
      break;
    case 0x61:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x19);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_9a = DAT_0065d424;
      uStack_9c = 0x160b;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_9c);
      break;
    case 0x62:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x1a);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_7c = 0x160c;
      uStack_7a = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_7c);
      break;
    case 99:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x1b);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_92 = DAT_0065d424;
      uStack_94 = 0x160d;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_94);
      break;
    case 100:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x1c);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_6c = 0x160e;
      uStack_6a = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_6c);
      break;
    case 0x65:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x1d);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_8a = DAT_0065d424;
      uStack_8c = 0x161c;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_8c);
      break;
    case 0x66:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x1e);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_74 = 0x161b;
      uStack_72 = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_74);
      break;
    case 0x67:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x1f);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_ae = DAT_0065d424;
      uStack_b0 = 0x161a;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_b0);
      break;
    case 0x71:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x20);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_a8 = 0x1610;
      uStack_a6 = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_a8);
      break;
    case 0x72:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x21);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      local_b8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1611);
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&local_b8);
      break;
    case 0x73:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x22);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_c8._0_2_ = 0x1612;
      uStack_c8._2_2_ = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_c8);
      break;
    case 0x74:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x23);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_c4 = (void *)CONCAT22(DAT_0065d424,0x1613);
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_c4);
      break;
    case 0x75:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x24);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_c0._0_2_ = 0x1619;
      uStack_c0._2_2_ = DAT_0065d424;
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_c0);
      break;
    case 0x91:
      RStack_f0.left = FUN_00618b70(0xc);
      local_4 = CONCAT31(local_4._1_3_,0x25);
      if ((void *)RStack_f0.left == (void *)0x0) goto LAB_00426a17;
      uStack_f4 = (void *)CONCAT22(DAT_0065d424,0x162a);
      puVar9 = (undefined4 *)FUN_005f2fc0((void *)RStack_f0.left,&uStack_f4);
    }
  }
  else {
    uStack_f4 = (void *)FUN_00618b70(0xc);
    local_4 = CONCAT31(local_4._1_3_,8);
    if (uStack_f4 == (void *)0x0) goto LAB_00426a17;
    puVar9 = (undefined4 *)FUN_005f2fc0(uStack_f4,&local_bc);
  }
  local_4 = local_4 & 0xffffff00;
switchD_0042622f_caseD_18:
  if (puVar9 != (undefined4 *)0x0) {
    FUN_00601880(aiStack_68,(char *)0x0,0,0,0,0,0,0);
    local_4._0_1_ = 0x26;
    iVar2 = *(int *)((int)this + 0xcc);
    iVar1 = *(int *)((int)this + 0xd4);
    RStack_f0.bottom = *(LONG *)((int)this + 0xd8);
    FUN_00601b30(aiStack_68,iVar2,*(int *)((int)this + 0xd0));
    aiStack_68[0] = iVar1 - iVar2;
    pcVar7 = (char *)FUN_00583c40((int)puVar9);
    FUN_00601aa0(aiStack_68,pcVar7);
    FUN_00403e90(aiStack_68,1);
    uStack_3c = 1;
    FUN_00601c90(aiStack_68,(-(uint)(*(int *)((int)this + 0x9c) != 1) & 0xfe01) + 0x20000ff,
                 (HWND)0x0);
    (**(code **)*puVar9)(1);
    pvVar8 = (HGDIOBJ)FUN_005fc130((int)local_dc);
    pvVar8 = SelectObject(*(HDC *)((int)this + 0x460),pvVar8);
    FUN_00601ce0(aiStack_68,*(HDC *)((int)this + 0x460));
    SelectObject(*(HDC *)((int)this + 0x460),pvVar8);
    RStack_f0.top = LStack_50;
    RStack_f0.right = iStack_4c;
    RStack_f0.left = (LONG)pvStack_54;
    RStack_f0.bottom = LStack_48;
    InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_f0,0);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00601990((int)aiStack_68);
  }
  FUN_00426d00(this,param_1);
  uStack_14 = uStack_14 & 0xfffffff9 | 1;
  uStack_1c = 7;
  uStack_20 = 0x259;
  FUN_00610c30((int)local_24);
  for (iVar2 = FUN_005f5060((int)this + 0x6c); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
    if (((byte)*(undefined4 *)(iVar2 + 0x24) & 0x3f) == 1) {
      InvalidateRect(*(HWND *)(iVar2 + 0x18),(RECT *)0x0,0);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

