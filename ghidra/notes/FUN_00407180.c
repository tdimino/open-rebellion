
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_00407180(void *param_1)

{
  bool bVar1;
  undefined1 uVar2;
  bool bVar3;
  undefined4 uVar4;
  uint *puVar5;
  void *pvVar6;
  int *piVar7;
  int iVar8;
  uint uVar9;
  char *pcVar10;
  uint *puVar11;
  undefined4 *this;
  undefined3 extraout_var;
  undefined4 uVar12;
  undefined4 uVar13;
  uint uVar14;
  uint uVar15;
  float10 fVar16;
  longlong lVar17;
  undefined8 uVar18;
  UINT UVar19;
  int local_ec;
  int *local_e0;
  int local_dc;
  undefined4 local_d4 [3];
  int local_c8 [27];
  undefined4 local_5c [6];
  int local_44;
  int local_3c;
  undefined4 auStack_34 [6];
  int iStack_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629a15;
  pvStack_c = ExceptionList;
  uVar12 = *(undefined4 *)((int)param_1 + 0x1c);
  ExceptionList = &pvStack_c;
  uVar4 = FUN_006037f0(8);
  puVar5 = (uint *)FUN_005ff440((int)param_1);
  FUN_005f2f50(local_d4);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  if ((*(int *)((int)param_1 + 0xa8) < 0x14) || (bVar1 = true, 0x17 < *(int *)((int)param_1 + 0xa8))
     ) {
    bVar1 = false;
  }
  local_c8[0x12] = 0x51;
  local_c8[0x13] = 0x7b;
  local_c8[0x16] = 0xf9;
  local_c8[0xc] = 0x51;
  local_c8[0xd] = 0x7b;
  local_c8[0x10] = 0xf9;
  local_c8[6] = 0x51;
  local_c8[7] = 0x7b;
  local_c8[10] = 0xf9;
  local_e0 = (int *)((int)param_1 + 0xc4);
  local_c8[0x14] = 0xa5;
  local_c8[0x15] = 0xcf;
  local_c8[0x17] = 0x123;
  local_c8[0xe] = 0xa5;
  local_c8[0xf] = 0xcf;
  local_c8[0x11] = 0x123;
  local_c8[0] = 0x53;
  local_c8[1] = 0x7d;
  local_c8[2] = 0xa7;
  local_c8[3] = 0xd1;
  local_c8[4] = 0xfb;
  local_c8[5] = 0x125;
  local_c8[8] = 0xa5;
  local_c8[9] = 0xcf;
  local_c8[0xb] = 0x123;
  local_ec = 1;
  local_dc = 0;
  do {
    pvVar6 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 1;
    if (pvVar6 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = (int *)FUN_00602150(pvVar6,uVar4,0x22,
                                   *(undefined4 *)((int)local_c8 + local_dc + 0x48),0x2a,0x14,
                                   param_1,local_ec + 0x6e,0x273e,0x273f,puVar5,0,0);
    }
    local_4._0_1_ = 0;
    if (piVar7 != (int *)0x0) {
      FUN_00603150(piVar7,2,0x2740);
      FUN_005ffce0(piVar7,0);
      FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar7);
      FUN_006009f0(param_1,piVar7,0x1010);
      if (((*(int *)((int)param_1 + 0xa8) == 0) || (bVar1)) ||
         ((DAT_006b1208 != 0 && (iVar8 = FUN_00401960(), iVar8 == 0)))) {
        (**(code **)(*piVar7 + 0x10))();
      }
    }
    pvVar6 = (void *)FUN_00618b70(0xf4);
    local_4._0_1_ = 2;
    if (pvVar6 == (void *)0x0) {
      pvVar6 = (void *)0x0;
    }
    else {
      pvVar6 = FUN_00604cf0(pvVar6,uVar12,0x77,*(undefined4 *)((int)local_c8 + local_dc + 0x18),0xa0
                            ,0x14,(int)param_1,puVar5,0);
    }
    local_4._0_1_ = 0;
    uVar2 = (undefined1)local_4;
    local_4._0_1_ = 0;
    *local_e0 = (int)pvVar6;
    if (pvVar6 != (void *)0x0) {
      *(undefined4 *)((int)pvVar6 + 0xe4) = 0x2ffffff;
      *(undefined4 *)(*local_e0 + 0xe8) = 0x2000000;
      *(int *)(*local_e0 + 0x24) = local_ec + 0x82;
      FUN_005ffce0((void *)*local_e0,0);
      if ((*(int *)((int)param_1 + 0x9c) == 4) || (*(int *)((int)param_1 + 0x9c) == 5)) {
        iVar8 = 1;
      }
      else {
        iVar8 = 0;
      }
      FUN_00401860(local_5c,local_ec,iVar8);
      local_4._0_1_ = 3;
      pvVar6 = FUN_00408d90(local_5c,local_c8 + 0x18);
      local_4._0_1_ = 4;
      FUN_005f3090(local_d4,(int)pvVar6);
      local_4._0_1_ = 3;
      FUN_005f2ff0(local_c8 + 0x18);
      uVar9 = FUN_005f3070((int)local_d4);
      if ((short)uVar9 == 0) {
        FUN_00604f90((void *)*local_e0,&DAT_006b120c);
      }
      else {
        pcVar10 = (char *)FUN_00583c40((int)local_d4);
        FUN_00604f90((void *)*local_e0,pcVar10);
        if (local_44 == 2) {
          uVar13 = 0x2749;
        }
        else if (local_3c == 1) {
          uVar13 = 0x2748;
        }
        else {
          if (local_3c != 2) goto LAB_0040753b;
          uVar13 = 0x2747;
        }
        iVar8 = *(int *)((int)local_c8 + local_dc);
        puVar11 = (uint *)FUN_005ff440((int)param_1);
        if (puVar11 != (uint *)0x0) {
          pvVar6 = (void *)FUN_00618b70(0x28);
          local_4._0_1_ = 5;
          if (pvVar6 == (void *)0x0) {
            this = (undefined4 *)0x0;
          }
          else {
            uVar18 = CONCAT44(2,uVar13);
            uVar13 = FUN_006037f0(8);
            this = (undefined4 *)
                   FUN_005fbd20(pvVar6,uVar13,(uint)uVar18,(char)((ulonglong)uVar18 >> 0x20));
          }
          local_4._0_1_ = 3;
          if (this != (undefined4 *)0x0) {
            FUN_005fd0f0(this,puVar11,0x55,iVar8);
            FUN_005fbfa0(this);
            FUN_00618b60((undefined *)this);
          }
        }
      }
LAB_0040753b:
      if (((*(int *)((int)param_1 + 0xa8) == 0) || (bVar1)) ||
         ((DAT_006b1208 != 0 && (iVar8 = FUN_00401960(), iVar8 == 0)))) {
        (**(code **)(*(int *)*local_e0 + 0x10))();
      }
      local_4._0_1_ = 0;
      FUN_004118f0(local_5c);
      uVar2 = (undefined1)local_4;
    }
    local_4._0_1_ = uVar2;
    pvVar6 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 6;
    if (pvVar6 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = (int *)FUN_00602150(pvVar6,uVar4,0x11f,
                                   *(undefined4 *)((int)local_c8 + local_dc + 0x30),0x29,0x14,
                                   param_1,local_ec + 0x78,0x2741,0x2742,puVar5,0,0);
    }
    local_4._0_1_ = 0;
    if (piVar7 != (int *)0x0) {
      FUN_00603150(piVar7,2,0x2743);
      FUN_005ffce0(piVar7,0);
      FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar7);
      if ((bVar1) || ((DAT_006b1208 != 0 && (iVar8 = FUN_00401960(), iVar8 == 0)))) {
        (**(code **)(*piVar7 + 0x10))();
      }
      FUN_00401860(auStack_34,local_ec,DAT_006b1208);
      local_4._0_1_ = 7;
      if (DAT_006b1208 == 0) {
        uVar9 = (uint)(iStack_1c == 1);
      }
      else {
        uVar9 = (uint)(iStack_1c == 2);
        if (uVar9 != 0) {
          bVar3 = FUN_00411410(local_ec);
          uVar9 = CONCAT31(extraout_var,bVar3);
        }
      }
      local_4._0_1_ = 0;
      FUN_004118f0(auStack_34);
      if (uVar9 == 0) {
        (**(code **)(*piVar7 + 0x10))();
      }
      else {
        if ((*(int *)((int)param_1 + 0x9c) == 4) || (*(int *)((int)param_1 + 0x9c) == 5)) {
          iVar8 = 1;
        }
        else {
          iVar8 = 0;
        }
        iVar8 = FUN_00401910(local_ec,iVar8);
        if (iVar8 == 0) {
          (**(code **)(*piVar7 + 0x10))();
        }
      }
      FUN_006009f0(param_1,piVar7,0x1011);
    }
    local_dc = local_dc + 4;
    local_e0 = local_e0 + 1;
    bVar3 = local_ec < 6;
    local_ec = local_ec + 1;
  } while (bVar3);
  pvVar6 = (void *)FUN_00618b70(0xe4);
  local_4._0_1_ = 8;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    iVar8 = FUN_00401a80();
    if (iVar8 < -0x2580) {
      uVar12 = 0;
    }
    else {
      fVar16 = (float10)FUN_006197b0();
      FUN_00619610(SUB84((double)(fVar16 * (float10)_DAT_006585f8),0),
                   (uint)((ulonglong)(double)(fVar16 * (float10)_DAT_006585f8) >> 0x20));
      lVar17 = __ftol();
      uVar12 = (undefined4)lVar17;
    }
    pvVar6 = FUN_00604570(pvVar6,uVar4,0x188,0xbe,param_1,0x8e,2,puVar5,0x2745,0x2746,uVar12,100);
  }
  local_4._0_1_ = 0;
  *(void **)((int)param_1 + 0xe0) = pvVar6;
  if (pvVar6 != (void *)0x0) {
    FUN_005ffce0(pvVar6,0);
    FUN_006009f0(param_1,*(void **)((int)param_1 + 0xe0),0x1013);
  }
  pvVar6 = (void *)FUN_00618b70(0xe4);
  local_4._0_1_ = 9;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    iVar8 = FUN_00401ac0();
    if (iVar8 < -0x2580) {
      uVar12 = 0;
    }
    else {
      fVar16 = (float10)FUN_006197b0();
      FUN_00619610(SUB84((double)(fVar16 * (float10)_DAT_006585f8),0),
                   (uint)((ulonglong)(double)(fVar16 * (float10)_DAT_006585f8) >> 0x20));
      lVar17 = __ftol();
      uVar12 = (undefined4)lVar17;
    }
    pvVar6 = FUN_00604570(pvVar6,uVar4,0x188,0x82,param_1,0x8d,2,puVar5,0x2744,0x2746,uVar12,100);
  }
  local_4._0_1_ = 0;
  *(void **)((int)param_1 + 0xdc) = pvVar6;
  if (pvVar6 != (void *)0x0) {
    FUN_005ffce0(pvVar6,0);
    FUN_006009f0(param_1,*(void **)((int)param_1 + 0xdc),0x1012);
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 10;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_00602150(pvVar6,uVar4,0x4c,0x17d,0x2a,0x2a,param_1,0x91,0x2733,0x2734,puVar5,0,0);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  if (pvVar6 != (void *)0x0) {
    FUN_005ffce0(pvVar6,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar6);
    FUN_006009f0(param_1,pvVar6,0x1014);
  }
  switch(*(undefined4 *)((int)param_1 + 0x9c)) {
  case 4:
  case 0xc:
    uVar15 = 0x272d;
    uVar9 = 0x272e;
    uVar14 = 0x272f;
    break;
  case 8:
    iVar8 = FUN_00401060();
    if (iVar8 != 1) {
      uVar15 = 0x2727;
      uVar9 = 0x2728;
      uVar14 = 0x2729;
      break;
    }
  default:
    uVar15 = 0x2724;
    uVar9 = 0x2725;
    uVar14 = 0x2726;
    break;
  case 9:
    uVar15 = 0x272a;
    uVar9 = 0x272b;
    uVar14 = 0x272c;
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xb;
  if (pvVar6 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_00602150(pvVar6,uVar4,0xa2,0x17e,0x2a,0x2a,param_1,0x90,uVar15,uVar9,puVar5,
                                 0,0);
  }
  local_4._0_1_ = 0;
  if (piVar7 != (int *)0x0) {
    FUN_00603150(piVar7,2,uVar14);
    FUN_005ffce0(piVar7,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar7);
    if ((*(int *)((int)param_1 + 0x9c) == 4) || (*(int *)((int)param_1 + 0x9c) == 5)) {
      UVar19 = 0x1019;
    }
    else {
      UVar19 = 0x1015;
    }
    FUN_006009f0(param_1,piVar7,UVar19);
    if (((*(int *)((int)param_1 + 0xa8) == 0) && (*(int *)((int)param_1 + 0x9c) != 4)) &&
       (*(int *)((int)param_1 + 0x9c) != 5)) {
      (**(code **)(*piVar7 + 0x10))();
    }
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xc;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_00602150(pvVar6,uVar4,0xf8,0x17d,0x2a,0x2a,param_1,0x8f,0x2736,0x2737,puVar5,0,0);
  }
  local_4._0_1_ = 0;
  if (pvVar6 != (void *)0x0) {
    FUN_005ffce0(pvVar6,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar6);
    FUN_006009f0(param_1,pvVar6,0x1018);
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xd;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_00602150(pvVar6,uVar4,0x160,0x4c,0x13,0x23,param_1,0x99,0x2738,0x2739,puVar5,0,0);
  }
  local_4._0_1_ = 0;
  if (pvVar6 != (void *)0x0) {
    FUN_00603150(pvVar6,2,0x273a);
    FUN_00603150(pvVar6,4,0x2739);
    FUN_006030c0(pvVar6,8);
    FUN_005ffce0(pvVar6,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar6);
    if ((*(uint *)((int)param_1 + 0xa0) & 0x100) != 0) {
      FUN_006030c0(pvVar6,4);
    }
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xe;
  if (pvVar6 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_00602150(pvVar6,uVar4,0x165,0x137,0x23,0x16,param_1,0xa1,0x273b,0x273c,
                                 puVar5,0,0);
  }
  local_4._0_1_ = 0;
  if (piVar7 != (int *)0x0) {
    FUN_00603150(piVar7,2,0x273d);
    FUN_00603150(piVar7,4,0x273c);
    FUN_006030c0(piVar7,8);
    FUN_005ffce0(piVar7,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar7);
    if (bVar1) {
      (**(code **)(*piVar7 + 0x10))();
    }
    else if ((*(uint *)((int)param_1 + 0xa0) & 0x10000000) != 0) {
      FUN_006030c0(piVar7,4);
    }
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xf;
  if (pvVar6 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_00602150(pvVar6,uVar4,0x165,0x151,0x23,0x16,param_1,0xa2,0x273b,0x273c,
                                 puVar5,0,0);
  }
  local_4._0_1_ = 0;
  if (piVar7 != (int *)0x0) {
    FUN_00603150(piVar7,2,0x273d);
    FUN_00603150(piVar7,4,0x273c);
    FUN_006030c0(piVar7,8);
    FUN_005ffce0(piVar7,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar7);
    if (bVar1) {
      (**(code **)(*piVar7 + 0x10))();
    }
    else if ((*(uint *)((int)param_1 + 0xa0) & 0x1000000) != 0) {
      FUN_006030c0(piVar7,4);
    }
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x10;
  if (pvVar6 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_00602150(pvVar6,uVar4,0x165,0x16d,0x23,0x16,param_1,0xa3,0x273b,0x273c,
                                 puVar5,0,0);
  }
  local_4._0_1_ = 0;
  if (piVar7 != (int *)0x0) {
    FUN_00603150(piVar7,2,0x273d);
    FUN_00603150(piVar7,4,0x273c);
    FUN_006030c0(piVar7,8);
    FUN_005ffce0(piVar7,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar7);
    if (bVar1) {
      (**(code **)(*piVar7 + 0x10))();
    }
    else if ((*(uint *)((int)param_1 + 0xa0) & 0x100000) != 0) {
      FUN_006030c0(piVar7,4);
    }
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x11;
  if (pvVar6 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_00602150(pvVar6,uVar4,0x165,0x188,0x23,0x16,param_1,0xa4,0x273b,0x273c,
                                 puVar5,0,0);
  }
  local_4._0_1_ = 0;
  if (piVar7 != (int *)0x0) {
    FUN_00603150(piVar7,2,0x273d);
    FUN_00603150(piVar7,4,0x273c);
    FUN_006030c0(piVar7,8);
    FUN_005ffce0(piVar7,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar7);
    if (bVar1) {
      (**(code **)(*piVar7 + 0x10))();
    }
    else if ((*(uint *)((int)param_1 + 0xa0) & 0x10000) != 0) {
      FUN_006030c0(piVar7,4);
    }
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x12;
  if (pvVar6 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_00602150(pvVar6,uVar4,0x165,0x1a3,0x23,0x16,param_1,0xa6,0x273b,0x273c,
                                 puVar5,0,0);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  if (piVar7 != (int *)0x0) {
    FUN_00603150(piVar7,2,0x273d);
    FUN_00603150(piVar7,4,0x273c);
    FUN_006030c0(piVar7,8);
    FUN_005ffce0(piVar7,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar7);
    if (bVar1) {
      (**(code **)(*piVar7 + 0x10))();
    }
    else if ((*(uint *)((int)param_1 + 0xa0) & 0x2000000) != 0) {
      FUN_006030c0(piVar7,4);
    }
  }
  FUN_00408f10((int)param_1);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_d4);
  ExceptionList = pvStack_c;
  return;
}

