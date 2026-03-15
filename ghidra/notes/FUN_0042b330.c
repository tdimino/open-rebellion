
/* WARNING: Removing unreachable block (ram,0x0042b81d) */
/* WARNING: Removing unreachable block (ram,0x0042b758) */

void __thiscall FUN_0042b330(void *this,uint *param_1,int *param_2,int param_3)

{
  byte bVar1;
  bool bVar2;
  bool bVar3;
  uint *puVar4;
  void *this_00;
  uint uVar5;
  void *pvVar6;
  int iVar7;
  int iVar8;
  undefined4 uVar9;
  int *piVar10;
  short sVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  undefined4 uVar16;
  uint local_138;
  uint local_134;
  uint local_130;
  int *local_12c;
  uint local_128;
  uint local_124;
  int local_120;
  int local_11c;
  int local_118;
  int local_114;
  uint local_110;
  undefined4 local_10c;
  int local_108;
  int local_104;
  int local_100;
  int local_fc;
  int local_f8;
  int local_f4;
  uint local_f0;
  undefined4 local_ec [7];
  int local_d0;
  undefined4 local_cc [7];
  int local_b0;
  undefined4 local_ac [7];
  int local_90;
  undefined4 local_8c [7];
  int local_70;
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c6d1;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_130);
  sVar11 = 0;
  uVar12 = 0;
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  if ((*(int *)((int)this + 200) == 0x80) ||
     ((piVar10 = param_2, param_2 == (int *)0x0 &&
      (param_2 = FUN_004f2d10(*(int *)((int)this + 0x9c),param_1), piVar10 = param_2,
      param_2 == (int *)0x0)))) goto LAB_0042c270;
  do {
    local_12c = piVar10;
    puVar4 = FUN_004025b0(param_2,&local_134);
    local_110 = 0x90;
    local_10c = 0x98;
    local_4._0_1_ = 1;
    if ((*puVar4 >> 0x18 < 0x90) || (0x97 < *puVar4 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    local_4._0_1_ = 0;
    FUN_00619730();
  } while ((!bVar2) && (param_2 = (int *)param_2[7], piVar10 = local_12c, param_2 != (int *)0x0));
  local_12c = param_2;
  if (param_2 == (int *)0x0) goto LAB_0042c270;
  puVar4 = FUN_004025b0(param_2,&local_134);
  local_4._0_1_ = 2;
  FUN_004f26d0(&local_130,puVar4);
  local_4._0_1_ = 0;
  FUN_00619730();
  uVar13 = (uint)param_2[9] >> 6 & 3;
  local_120 = 0;
  this_00 = (void *)FUN_0060a860((void *)((int)this + 0x484),local_130 & 0xffffff);
  iVar7 = *(int *)((int)this + 200);
  if ((iVar7 == 0x22) || (iVar7 == 0x44)) {
    bVar2 = true;
  }
  else {
    bVar2 = false;
  }
  if ((((iVar7 == 0x22) || (iVar7 == 0x20)) || (iVar7 == 0x21)) ||
     (((iVar7 == 0x43 || (iVar7 == 0x44)) || ((iVar7 == 0x92 || (iVar7 == 0x93)))))) {
    bVar3 = true;
  }
  else {
    bVar3 = false;
  }
  if (iVar7 == 0x17) {
    bVar1 = *(byte *)(param_2 + 0x22);
  }
  else {
    bVar1 = *(byte *)(param_2 + 0x22);
  }
  if ((((bVar1 & 2) != 0) || ((!bVar3 && ((*(byte *)(param_2 + 0x14) & 8) != 0)))) && (!bVar2)) {
    *(uint *)((int)this_00 + 0x3c) = *(uint *)((int)this_00 + 0x3c) | 0x80;
    iVar7 = 1;
    goto LAB_0042c1c3;
  }
  sVar11 = 0;
  switch(iVar7) {
  case 0x10:
  case 0x11:
    uVar12 = FUN_00507270(param_2,*(int *)((int)this + 0x9c));
    goto LAB_0042b663;
  case 0x12:
    iVar15 = 1;
    iVar14 = 1;
    iVar8 = 1;
    uVar12 = (uint)param_2[0x22] >> 2 & 1;
    iVar7 = 1;
    goto LAB_0042c0bf;
  case 0x13:
    if ((uVar13 != *(uint *)((int)this + 0x9c)) ||
       (((uint)param_2[0x21] >> 2 & 3) != *(uint *)((int)this + 0x9c))) goto LAB_0042b651;
    uVar12 = 0x52;
    goto LAB_0042b663;
  case 0x14:
    if ((uVar13 != *(uint *)((int)this + 0x9c)) ||
       (((uint)param_2[0x21] >> 2 & 3) == *(uint *)((int)this + 0x9c))) goto LAB_0042b651;
    uVar12 = 0x52;
    goto LAB_0042b663;
  case 0x15:
    iVar7 = *(int *)((int)this + 0x9c);
    if (iVar7 == 3) {
      uVar12 = 3;
    }
    else {
      uVar12 = 2 - (iVar7 != 1);
    }
    if (uVar13 == uVar12) {
      if (iVar7 == 3) {
        uVar12 = 3;
      }
      else {
        uVar12 = 2 - (iVar7 != 1);
      }
      if (((uint)param_2[0x21] >> 2 & 3) == uVar12) {
        uVar12 = 0x52;
        goto LAB_0042b663;
      }
    }
    goto LAB_0042b651;
  case 0x16:
    iVar7 = *(int *)((int)this + 0x9c);
    if (iVar7 == 3) {
      uVar12 = 3;
    }
    else {
      uVar12 = 2 - (iVar7 != 1);
    }
    if (uVar13 == uVar12) {
      if (iVar7 == 3) {
        uVar12 = 3;
      }
      else {
        uVar12 = 2 - (iVar7 != 1);
      }
      if (((uint)param_2[0x21] >> 2 & 3) != uVar12) {
        uVar12 = 0x52;
        goto LAB_0042b663;
      }
    }
LAB_0042b651:
    uVar12 = 0;
LAB_0042b663:
    uVar9 = FUN_00426e70(this,uVar12,uVar13,0x51,0x50,0x3c,0x3b,0x32);
    sVar11 = (short)uVar9;
    bVar1 = *(byte *)((int)this_00 + 0x3c);
LAB_0042b6a9:
    if ((bVar1 & 0x80) != 0) {
      *(uint *)((int)this_00 + 0x54) = *(uint *)((int)this_00 + 0x54) & 0xffffff7f;
    }
    goto switchD_0042b525_caseD_18;
  case 0x17:
    uVar12 = (-(uint)((param_2[0x22] & 2U) != 0) & 0xffffffae) + 0x52;
    uVar9 = FUN_00426e70(this,uVar12,3,0x51,0x50,0x3c,0x3b,0x32);
    sVar11 = (short)uVar9;
    bVar1 = *(byte *)((int)this_00 + 0x3c);
    goto LAB_0042b6a9;
  default:
    goto switchD_0042b525_caseD_18;
  case 0x20:
  case 0x21:
    sVar11 = 0;
    if (*(int *)((int)this + 0x9c) == 1) {
      uVar13 = 1;
    }
    else if (*(int *)((int)this + 0x9c) == 2) {
      uVar13 = 2;
    }
    else {
      uVar13 = 0;
    }
    uVar12 = FUN_004feea0(param_2,uVar13,1);
    if (*(int *)((int)this + 0x9c) == 1) {
      uVar9 = 2;
    }
    else if (*(int *)((int)this + 0x9c) == 2) {
      uVar9 = 1;
    }
    else {
      uVar9 = 0;
    }
    uVar5 = FUN_004feea0(param_2,uVar9,1);
    if (uVar5 == 0) {
LAB_0042b821:
      if (uVar12 == 0) {
        uVar13 = (uint)param_2[9] >> 6 & 3;
        uVar5 = FUN_004feea0(param_2,uVar13,1);
LAB_0042b83a:
        sVar11 = 0;
        uVar12 = uVar5;
      }
    }
    else {
      if (uVar12 == 0) {
        if (uVar5 == 0) goto LAB_0042b821;
LAB_0042b7fb:
        if (*(int *)((int)this + 0x9c) == 1) {
          uVar13 = 2;
        }
        else if (*(int *)((int)this + 0x9c) == 2) {
          uVar13 = 1;
        }
        else {
          uVar13 = 0;
        }
        goto LAB_0042b83a;
      }
      uVar12 = uVar12 + uVar5;
      sVar11 = 0x27b0;
    }
    goto LAB_0042b83c;
  case 0x22:
    sVar11 = 0;
    if (*(int *)((int)this + 0x9c) == 1) {
      uVar13 = 1;
    }
    else if (*(int *)((int)this + 0x9c) == 2) {
      uVar13 = 2;
    }
    else {
      uVar13 = 0;
    }
    uVar12 = FUN_004feea0(param_2,uVar13,2);
    if (*(int *)((int)this + 0x9c) == 1) {
      uVar9 = 2;
    }
    else if (*(int *)((int)this + 0x9c) == 2) {
      uVar9 = 1;
    }
    else {
      uVar9 = 0;
    }
    uVar5 = FUN_004feea0(param_2,uVar9,2);
    if (uVar5 != 0) {
      if (uVar12 != 0) {
        sVar11 = 0x27b0;
        uVar12 = uVar12 + uVar5;
        goto LAB_0042b83c;
      }
      if (uVar5 != 0) goto LAB_0042b7fb;
    }
    if (uVar12 == 0) {
      if ((*(byte *)(param_2 + 0x22) & 2) == 0) {
        uVar13 = 0;
      }
      else {
        uVar13 = (uint)param_2[9] >> 6 & 3;
      }
    }
LAB_0042b83c:
    if (sVar11 == 0) {
      iVar15 = 1;
      iVar14 = 2;
      iVar8 = 2;
      iVar7 = 3;
      goto LAB_0042c0bf;
    }
    goto switchD_0042b525_caseD_18;
  case 0x43:
  case 0x44:
    uVar12 = 0;
    iVar7 = 0;
    local_11c = 0;
    local_134 = 0;
    local_108 = 0;
    local_124 = 0;
    local_f4 = 0;
    local_f8 = 0;
    local_114 = 0;
    local_104 = 0;
    local_fc = 0;
    local_118 = 0;
    local_100 = 0;
    if (*(int *)((int)this + 0x9c) == 1) {
      uVar13 = 1;
    }
    else if (*(int *)((int)this + 0x9c) == 2) {
      uVar13 = 2;
    }
    else {
      uVar13 = 0;
    }
    FUN_004f2640(local_cc,param_2,3,uVar13);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00513120((int)local_cc);
    uVar5 = local_134;
    while (local_b0 != 0) {
      iVar8 = FUN_0052bed0((int)local_cc);
      local_134 = *(uint *)(iVar8 + 0x50) & 1;
      if (local_134 == 0) {
LAB_0042b953:
        local_11c = local_11c + 1;
      }
      else {
        iVar8 = FUN_0052bed0((int)local_cc);
        local_134 = *(uint *)(iVar8 + 0x78) >> 7 & 1;
        if (local_134 != 0) goto LAB_0042b953;
        iVar8 = FUN_0052bed0((int)local_cc);
        local_134 = *(uint *)(iVar8 + 0xac) & 1;
        if (((local_134 != 0) ||
            (iVar8 = FUN_0052bed0((int)local_cc), *(short *)(iVar8 + 0x94) != 0)) ||
           (iVar8 = FUN_0052bed0((int)local_cc), *(short *)(iVar8 + 0x96) != 0)) goto LAB_0042b953;
        uVar12 = uVar12 + 1;
      }
      FUN_005130d0((int)local_cc);
      uVar5 = uVar12;
    }
    local_134 = uVar5;
    FUN_005040c0(local_6c,param_2,3,uVar13);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00513120((int)local_6c);
    uVar12 = local_124;
    while (local_50 != 0) {
      iVar8 = FUN_0052bed0((int)local_6c);
      local_124 = *(uint *)(iVar8 + 0x50) & 1;
      if (local_124 == 0) {
LAB_0042b9df:
        local_108 = local_108 + 1;
      }
      else {
        iVar8 = FUN_0052bed0((int)local_6c);
        local_124 = *(uint *)(iVar8 + 0x78) >> 7 & 1;
        if (local_124 != 0) goto LAB_0042b9df;
        uVar12 = uVar12 + 1;
      }
      FUN_005130d0((int)local_6c);
    }
    local_124 = uVar12;
    FUN_004ffe70(local_4c,param_2,3);
    local_4 = CONCAT31(local_4._1_3_,5);
    FUN_00513120((int)local_4c);
    while (local_30 != 0) {
      uVar16 = 3;
      uVar9 = FUN_0052bed0((int)local_4c);
      FUN_00502db0(local_8c,uVar9,uVar16);
      local_4 = CONCAT31(local_4._1_3_,6);
      FUN_00513120((int)local_8c);
      while (local_70 != 0) {
        uVar16 = 3;
        uVar9 = FUN_0052bed0((int)local_8c);
        FUN_004f25c0(local_ec,uVar9,uVar16);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00513120((int)local_ec);
        while (local_d0 != 0) {
          iVar8 = FUN_0052bed0((int)local_ec);
          local_110 = *(uint *)(iVar8 + 0x50) & 1;
          if (local_110 == 0) {
LAB_0042bb1a:
            bVar2 = true;
          }
          else {
            iVar8 = FUN_0052bed0((int)local_ec);
            local_110 = *(uint *)(iVar8 + 0x78) >> 7 & 1;
            if (local_110 != 0) goto LAB_0042bb1a;
            iVar8 = FUN_0052bed0((int)local_ec);
            local_110 = *(uint *)(iVar8 + 0xac) & 1;
            if (((local_110 != 0) ||
                (iVar8 = FUN_0052bed0((int)local_ec), *(short *)(iVar8 + 0x94) != 0)) ||
               (iVar8 = FUN_0052bed0((int)local_ec), *(short *)(iVar8 + 0x96) != 0))
            goto LAB_0042bb1a;
            bVar2 = false;
          }
          iVar8 = FUN_0052bed0((int)local_ec);
          uVar12 = FUN_004edc80(iVar8);
          if (uVar13 == uVar12) {
            if (bVar2) {
              local_f4 = local_f4 + 1;
            }
            else {
              local_f8 = local_f8 + 1;
            }
          }
          else if (bVar2) {
            local_fc = local_fc + 1;
          }
          else {
            local_118 = local_118 + 1;
          }
          FUN_005130d0((int)local_ec);
        }
        uVar16 = 3;
        uVar12 = uVar13;
        uVar9 = FUN_0052bed0((int)local_8c);
        FUN_005040c0(local_ac,uVar9,uVar16,uVar12);
        local_4 = CONCAT31(local_4._1_3_,8);
        FUN_00513120((int)local_ac);
        while (local_90 != 0) {
          iVar8 = FUN_0052bed0((int)local_ac);
          local_110 = *(uint *)(iVar8 + 0x50) & 1;
          if (local_110 == 0) {
LAB_0042bbdf:
            bVar2 = true;
          }
          else {
            iVar8 = FUN_0052bed0((int)local_ac);
            local_110 = *(uint *)(iVar8 + 0x78) >> 7 & 1;
            if (local_110 != 0) goto LAB_0042bbdf;
            bVar2 = false;
          }
          iVar8 = FUN_0052bed0((int)local_ac);
          if (uVar13 == (*(uint *)(iVar8 + 0x24) >> 6 & 3)) {
            if (bVar2) {
              local_114 = local_114 + 1;
            }
            else {
              local_104 = local_104 + 1;
            }
          }
          else if (bVar2) {
            local_100 = local_100 + 1;
          }
          else {
            iVar7 = iVar7 + 1;
          }
          FUN_005130d0((int)local_ac);
        }
        local_4._1_3_ = (uint3)((uint)local_4 >> 8);
        local_4._0_1_ = 7;
        FUN_00504140(local_ac);
        local_4 = CONCAT31(local_4._1_3_,6);
        FUN_004f26c0(local_ec);
        FUN_005130d0((int)local_8c);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,5);
      FUN_00502eb0(local_8c);
      FUN_005130d0((int)local_4c);
    }
    if (*(int *)((int)this + 200) == 0x43) {
      iVar8 = local_104 + local_f8 + local_124 + local_134;
      iVar7 = iVar7 + local_118;
    }
    else {
      iVar8 = local_114 + local_f4 + local_108 + local_11c;
      iVar7 = local_100 + local_fc;
    }
    if (iVar8 == 0) {
      if (iVar7 == 0) {
        uVar13 = (uint)param_2[9] >> 6 & 3;
      }
      else if (*(int *)((int)this + 0x9c) == 1) {
LAB_0042bd2f:
        uVar13 = 2;
      }
      else {
        if (*(int *)((int)this + 0x9c) == 2) goto LAB_0042bd28;
        uVar13 = 0;
      }
    }
    else if (*(int *)((int)this + 0x9c) == 1) {
LAB_0042bd28:
      uVar13 = 1;
    }
    else {
      if (*(int *)((int)this + 0x9c) == 2) goto LAB_0042bd2f;
      uVar13 = 0;
    }
    uVar12 = iVar8 + iVar7;
    uVar9 = FUN_00426e70(this,uVar12,uVar13,1,1,1,1,1);
    sVar11 = (short)uVar9;
    local_4._0_1_ = 4;
    FUN_004fff70(local_4c);
    local_4._0_1_ = 3;
    FUN_00504140(local_6c);
    local_4._0_1_ = 0;
    FUN_004f26c0(local_cc);
    goto switchD_0042b525_caseD_18;
  case 0x51:
    uVar12 = param_2[0x17] - param_2[0x18];
    break;
  case 0x52:
    iVar15 = 1;
    iVar14 = 2;
    iVar8 = 2;
    uVar12 = param_2[0x19] - param_2[0x1a];
    iVar7 = 3;
    goto LAB_0042c0bf;
  case 0x53:
    uVar12 = FUN_0052d370(param_2,uVar13,1);
    break;
  case 0x54:
    uVar12 = FUN_0052cf40(param_2,uVar13,1);
    break;
  case 0x61:
    iVar7 = FUN_0052cc20(param_2,uVar13,1);
    iVar15 = 1;
    uVar12 = (uint)(0 < iVar7);
    iVar14 = 1;
    iVar8 = 1;
    iVar7 = 1;
    goto LAB_0042c0bf;
  case 0x62:
    uVar12 = FUN_0052c8c0(param_2,uVar13,1);
    iVar15 = 1;
    iVar14 = 2;
    iVar8 = 4;
    iVar7 = 5;
    goto LAB_0042c0bf;
  case 99:
    uVar12 = FUN_0052c5a0(param_2,uVar13,1);
    iVar15 = 1;
    iVar14 = 2;
    iVar8 = 4;
    iVar7 = 5;
    goto LAB_0042c0bf;
  case 100:
    uVar12 = FUN_0052c270(param_2,uVar13,1);
    iVar15 = 1;
    iVar14 = 2;
    iVar8 = 4;
    iVar7 = 5;
    goto LAB_0042c0bf;
  case 0x65:
    pvVar6 = (void *)FUN_00509670(param_2,0);
    if (((pvVar6 != (void *)0x0) && (iVar7 = FUN_00528040(pvVar6,0), iVar7 != 0)) &&
       (*(int *)((int)pvVar6 + 0x58) == 0)) {
      iVar7 = *(int *)((int)this + 0x9c);
LAB_0042beaf:
      if (iVar7 == 1) {
        uVar12 = 1;
      }
      else if (iVar7 == 2) {
        uVar12 = 2;
      }
      else {
        uVar12 = 0;
      }
      if (uVar13 == uVar12) {
        iVar15 = 1;
        iVar14 = 2;
        iVar8 = 4;
        uVar12 = 0xf;
        iVar7 = 5;
        goto LAB_0042c0bf;
      }
    }
    goto LAB_0042bedd;
  case 0x66:
    pvVar6 = (void *)FUN_00509670(param_2,2);
    if (((pvVar6 != (void *)0x0) && (iVar7 = FUN_00528040(pvVar6,0), iVar7 != 0)) &&
       (*(int *)((int)pvVar6 + 0x58) == 0)) {
      iVar7 = *(int *)((int)this + 0x9c);
      goto LAB_0042beaf;
    }
LAB_0042bedd:
    iVar15 = 1;
    iVar14 = 2;
    iVar8 = 4;
    uVar12 = 0;
    iVar7 = 5;
    goto LAB_0042c0bf;
  case 0x67:
    pvVar6 = (void *)FUN_00509670(param_2,1);
    if (((pvVar6 != (void *)0x0) && (iVar7 = FUN_00528040(pvVar6,0), iVar7 != 0)) &&
       (*(int *)((int)pvVar6 + 0x58) == 0)) {
      if (*(int *)((int)this + 0x9c) == 1) {
        uVar12 = 1;
      }
      else if (*(int *)((int)this + 0x9c) == 2) {
        uVar12 = 2;
      }
      else {
        uVar12 = 0;
      }
      if (uVar13 == uVar12) {
        iVar15 = 1;
        iVar14 = 3;
        iVar8 = 4;
        uVar12 = 0xf;
        iVar7 = 5;
        goto LAB_0042c0bf;
      }
    }
    iVar15 = 1;
    iVar14 = 3;
    iVar8 = 4;
    uVar12 = 0;
    iVar7 = 5;
    goto LAB_0042c0bf;
  case 0x71:
    uVar12 = FUN_005044f0(param_2,uVar13,1);
    break;
  case 0x72:
    uVar12 = FUN_00503550(param_2,uVar13,1);
    break;
  case 0x73:
    uVar12 = FUN_0051c0d0(param_2,uVar13,1);
    iVar15 = 1;
    iVar14 = 1;
    iVar8 = 1;
    iVar7 = 1;
    goto LAB_0042c0bf;
  case 0x74:
    uVar12 = FUN_005276d0(param_2,uVar13,1);
    break;
  case 0x75:
    iVar7 = FUN_005273d0(param_2,uVar13,1);
    iVar8 = FUN_00527150(param_2,uVar13,1);
    uVar12 = iVar7 + iVar8;
    break;
  case 0x91:
    iVar7 = FUN_005044f0(param_2,uVar13,1);
    iVar8 = FUN_00526e90(param_2,uVar13,1);
    iVar14 = FUN_00503550(param_2,uVar13,1);
    uVar12 = iVar7 + iVar8 + iVar14;
    break;
  case 0x92:
    puVar4 = FUN_004025b0(param_2,&local_f0);
    local_4._0_1_ = 9;
    iVar7 = FUN_005f5500((void *)((int)this + 0x2cc),*puVar4 & 0xffffff);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (iVar7 == 0) {
LAB_0042c09c:
      uVar12 = 0;
      uVar13 = 0;
      local_120 = 1;
      *(uint *)((int)this_00 + 0x3c) = *(uint *)((int)this_00 + 0x3c) | 0x80;
    }
    else {
      uVar12 = 7;
      uVar13 = 2;
    }
    break;
  case 0x93:
    puVar4 = FUN_004025b0(param_2,&local_128);
    local_4._0_1_ = 10;
    iVar7 = FUN_005f5500((void *)((int)this + 0x2cc),*puVar4 & 0xffffff);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (iVar7 == 0) goto LAB_0042c09c;
    uVar12 = 7;
    uVar13 = 1;
  }
  iVar15 = 2;
  iVar14 = 3;
  iVar8 = 5;
  iVar7 = 6;
LAB_0042c0bf:
  uVar9 = FUN_00426e70(this,uVar12,uVar13,iVar7,iVar8,iVar14,iVar15,1);
  sVar11 = (short)uVar9;
switchD_0042b525_caseD_18:
  if ((*(uint *)((int)this_00 + 0x54) != uVar12) && (sVar11 != 0)) {
    *(uint *)((int)this_00 + 0x54) = uVar12;
    local_120 = 1;
  }
  FUN_005f4950(&local_138,0);
  local_4._0_1_ = 0xb;
  FUN_00526cf0(local_2c,local_12c,3);
  local_4 = CONCAT31(local_4._1_3_,0xc);
  FUN_00513120((int)local_2c);
  if (local_10 != 0) {
    local_138 = local_138 | 8;
  }
  switch(uVar13) {
  case 0:
    local_138 = local_138 | 0x80;
    break;
  case 1:
    local_138 = local_138 | 0x10;
    break;
  case 2:
    local_138 = local_138 | 0x20;
    break;
  case 3:
    local_138 = local_138 | 0x40;
  }
  iVar7 = FUN_005f4960(&local_138);
  iVar8 = FUN_005f4960((undefined4 *)((int)this_00 + 0x3c));
  if (iVar7 != iVar8) {
    local_120 = 1;
    uVar9 = FUN_005f4960(&local_138);
    *(undefined4 *)((int)this_00 + 0x3c) = uVar9;
  }
  local_4._0_1_ = 0xb;
  FUN_00526df0(local_2c);
  local_4._0_1_ = 0;
  FUN_00619730();
  iVar7 = local_120;
  param_2 = local_12c;
LAB_0042c1c3:
  pvVar6 = (void *)param_2[7];
  if (((iVar7 != 0) && (this_00 != (void *)0x0)) && (pvVar6 != (void *)0x0)) {
    uVar9 = FUN_006037f0(7);
    FUN_0060bd20(this_00,sVar11,0,uVar9);
    if (param_3 != 0) {
      FUN_0042cde0(this,(uint)pvVar6);
    }
    puVar4 = FUN_004025b0(pvVar6,&local_128);
    uVar12 = *puVar4;
    FUN_00619730();
    piVar10 = (int *)FUN_00604500((void *)((int)this + 0x6c),(ushort)((short)uVar12 << 6) | 1);
    if (piVar10 != (int *)0x0) {
      puVar4 = FUN_004025b0(param_2,&local_128);
      local_4._0_1_ = 0xd;
      (**(code **)(*piVar10 + 100))(puVar4,param_2);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
  }
LAB_0042c270:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

