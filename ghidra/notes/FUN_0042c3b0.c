
uint * __cdecl FUN_0042c3b0(undefined4 param_1,uint *param_2,int *param_3,int param_4,int param_5)

{
  bool bVar1;
  undefined4 uVar2;
  uint *puVar3;
  uint uVar4;
  uint uVar5;
  undefined3 extraout_var;
  void *pvVar6;
  int iVar7;
  int iVar8;
  undefined4 *this;
  int *piVar9;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined4 uVar10;
  uint uVar11;
  uint uVar12;
  undefined1 uVar13;
  uint *puVar14;
  uint *puStack_24;
  int iStack_20;
  uint *local_1c;
  undefined4 uStack_18;
  undefined1 *puStack_14;
  undefined4 uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c750;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar2 = FUN_006037f0(7);
  if (param_3 == (int *)0x0) {
    iVar7 = *(int *)(DAT_006b14cc + 0x9c);
LAB_0042c42d:
    param_3 = FUN_004f2d10(iVar7,param_2);
  }
  else {
    puVar3 = FUN_004025b0(param_3,(uint *)&local_1c);
    uVar11 = *puVar3;
    uVar4 = *param_2;
    FUN_00619730();
    if (uVar11 != uVar4) {
      iVar7 = *(int *)(DAT_006b14cc + 0x9c);
      goto LAB_0042c42d;
    }
  }
  uVar11 = *param_2 >> 0x18;
  if (param_3 == (int *)0x0) {
    ExceptionList = local_c;
    return (uint *)0x0;
  }
  uVar4 = (**(code **)(*param_3 + 0x30))();
  FUN_00402d80(param_3,&iStack_20);
  iStack_4 = 0;
  puStack_14 = (undefined1 *)0x40;
  uStack_10 = 0x80;
  if ((uVar11 < 0x40) || (0x7f < uVar11)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  uVar10 = param_1;
  if (bVar1) {
    uVar12 = uVar4 & 0xfff;
    uVar5 = (uint)param_3[9] >> 6 & 3;
    if (uVar5 == 1) {
      uVar12 = uVar12 + (-(uint)(param_4 != 0) & 0xffffc000) + 0x4000;
    }
    else if (uVar5 == 2) {
      uVar12 = uVar12 + (-(uint)(param_4 != 0) & 0xffffc000) + 0x5000;
    }
  }
  else {
    puStack_14 = (undefined1 *)0x8;
    uStack_10 = 0x10;
    if ((uVar11 < 8) || (0xf < uVar11)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      uVar10 = uVar2;
      if (((uint)param_3[9] >> 6 & 3) == 1) {
        uVar12 = (-(uint)(param_4 != 0) & 0x19) + 0x28a0;
      }
      else {
        uVar12 = (-(uint)(param_4 != 0) & 0x19) + 0x28d2;
      }
    }
    else if (param_4 == 0) {
      uVar12 = (uVar4 & 0xfff) + 0x4000;
    }
    else {
      uVar12 = uVar4 & 0xfff;
    }
  }
  puStack_14 = (undefined1 *)FUN_00618b70(0x28);
  iStack_4._0_1_ = 1;
  if (puStack_14 == (undefined1 *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = (uint *)FUN_005fbd20(puStack_14,uVar10,uVar12,10);
  }
  iStack_4._0_1_ = 0;
  local_1c = puVar3;
  bVar1 = FUN_005fc040((int *)puVar3);
  if (CONCAT31(extraout_var,bVar1) == 0) {
    puStack_24 = (uint *)0x0;
    if (puVar3 != (uint *)0x0) {
      FUN_005fbfa0(puVar3);
      FUN_00618b60((undefined *)puVar3);
    }
    local_1c = (uint *)0x0;
    goto LAB_0042ccc2;
  }
  pvVar6 = (void *)FUN_00618b70(0x28);
  iStack_4._0_1_ = 2;
  if (pvVar6 == (void *)0x0) {
    puStack_24 = (uint *)0x0;
    puStack_14 = (undefined1 *)pvVar6;
  }
  else {
    puVar14 = puVar3;
    puStack_14 = (undefined1 *)pvVar6;
    iVar7 = FUN_005fc0f0((int *)puVar3);
    iVar8 = FUN_005fc0e0((int *)puVar3);
    puStack_24 = (uint *)FUN_005fbda0(pvVar6,iVar8,iVar7,puVar14);
  }
  uVar13 = (undefined1)puVar3[2];
  iStack_4._0_1_ = 0;
  iVar7 = FUN_005fc0f0((int *)puStack_24);
  uVar12 = FUN_005fc0e0((int *)puStack_24);
  FUN_005fcfe0(puStack_24,0,0,uVar12,iVar7,uVar13);
  puStack_14 = (undefined1 *)0x30;
  uStack_10 = 0x40;
  if (((uVar11 < 0x30) || (0x3f < uVar11)) || (param_4 != 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    puStack_14 = (undefined1 *)FUN_00618b70(0x28);
    iStack_4._0_1_ = 3;
    if (puStack_14 == (undefined1 *)0x0) {
      this = (undefined4 *)0x0;
    }
    else {
      this = (undefined4 *)FUN_005fbd20(puStack_14,uVar2,0x2cec,10);
    }
    iStack_4._0_1_ = 0;
    FUN_005fcc30(this,puStack_24,0,0,0,0,(undefined4 *)0x0,0);
joined_r0x0042c74f:
    if (this != (undefined4 *)0x0) {
      FUN_005fbfa0(this);
      FUN_00618b60((undefined *)this);
    }
  }
  else {
    puStack_14 = (undefined1 *)0x10;
    uStack_10 = 0x14;
    if ((uVar11 < 0x10) || (0x13 < uVar11)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      puStack_14 = (undefined1 *)FUN_00618b70(0x28);
      iStack_4._0_1_ = 4;
      if (puStack_14 == (undefined1 *)0x0) {
        this = (undefined4 *)0x0;
      }
      else {
        this = (undefined4 *)FUN_005fbd20(puStack_14,uVar2,(param_4 != 0) + 0x2cf9,10);
      }
      iStack_4._0_1_ = 0;
      FUN_005fcc30(this,puStack_24,0,0,0,0,(undefined4 *)0x0,0);
      goto joined_r0x0042c74f;
    }
  }
  if ((((param_3[0x14] & 0x10U) != 0) && ((param_3[0x14] & 8U) == 0)) && (param_5 != 0)) {
    if (param_4 == 0) {
      uVar12 = (uVar4 & 0xfff) + 0x5000;
    }
    else {
      uVar12 = (uVar4 & 0xfff) + 0x1000;
    }
    puStack_14 = (undefined1 *)0x3c;
    uStack_10 = 0x40;
    if ((uVar11 < 0x3c) || (0x3f < uVar11)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    uVar10 = param_1;
    if (bVar1) {
      puStack_14 = (undefined1 *)0x3c000003;
      iStack_4 = CONCAT31(iStack_4._1_3_,5);
      if (iStack_20 == 0x3c000003) {
LAB_0042c81b:
        bVar1 = false;
      }
      else {
        bVar1 = iStack_20 == 0x3c000005;
        uStack_18 = 0x3c000005;
        FUN_00619730();
        if (bVar1) goto LAB_0042c81b;
        bVar1 = true;
      }
      iStack_4._0_1_ = 0;
      FUN_00619730();
      if (bVar1) {
        uVar12 = (-(uint)(param_4 != 0) & 0x13) + 0x2ced;
        uVar10 = uVar2;
      }
    }
    puStack_14 = (undefined1 *)0x30;
    uStack_10 = 0x3c;
    if ((uVar11 < 0x30) || (0x3b < uVar11)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      uVar12 = (-(uint)(param_4 != 0) & 0xffffd313) + 0x2ced;
      uVar10 = uVar2;
    }
    else {
      puStack_14 = (undefined1 *)0x22;
      uStack_10 = 0x28;
      if ((uVar11 < 0x22) || (0x27 < uVar11)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        uVar12 = (param_4 != 0) + 0x2cf5;
        uVar10 = uVar2;
      }
      else {
        puStack_14 = &DAT_00000020;
        uStack_10 = 0x30;
        if ((uVar11 < 0x20) || (0x2f < uVar11)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          uVar12 = (param_4 != 0) + 0x2cf1;
          uVar10 = uVar2;
        }
        else {
          puStack_14 = (undefined1 *)0x10;
          uStack_10 = 0x14;
          if ((uVar11 < 0x10) || (0x13 < uVar11)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            uStack_18 = 0x10000002;
            iStack_4 = CONCAT31(iStack_4._1_3_,6);
            if (iStack_20 == 0x10000002) {
LAB_0042c999:
              bVar1 = false;
            }
            else {
              puStack_14 = (undefined1 *)0x10000008;
              FUN_00619730();
              if (iStack_20 == 0x10000008) goto LAB_0042c999;
              bVar1 = true;
            }
            iStack_4._0_1_ = 0;
            FUN_00619730();
            if (bVar1) {
              uVar12 = (param_4 != 0) + 0x2cfb;
              uVar10 = uVar2;
            }
          }
          else {
            puStack_14 = (undefined1 *)0x8;
            uStack_10 = 0x10;
            if ((uVar11 < 8) || (0xf < uVar11)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (bVar1) {
              if (((uint)param_3[9] >> 6 & 3) == 1) {
                uVar12 = (-(uint)(param_4 != 0) & 3) + 0x28b7;
                uVar10 = uVar2;
              }
              else {
                uVar12 = (-(uint)(param_4 != 0) & 3) + 0x28e9;
                uVar10 = uVar2;
              }
            }
          }
        }
      }
    }
    if ((short)uVar12 != 0) {
      puStack_14 = (undefined1 *)FUN_00618b70(0x28);
      iStack_4._0_1_ = 7;
      if (puStack_14 == (undefined1 *)0x0) {
        piVar9 = (int *)0x0;
      }
      else {
        piVar9 = (int *)FUN_005fbd20(puStack_14,uVar10,uVar12,10);
      }
      iStack_4._0_1_ = 0;
      bVar1 = FUN_005fc040(piVar9);
      if (CONCAT31(extraout_var_00,bVar1) != 0) {
        FUN_005fd0f0(piVar9,puStack_24,0,0);
      }
      if (piVar9 != (int *)0x0) {
        FUN_005fbfa0(piVar9);
        FUN_00618b60((undefined *)piVar9);
      }
    }
  }
  if (((param_3[0x14] & 0x200U) != 0) && (param_5 != 0)) {
    if (param_4 == 0) {
      uVar12 = (uVar4 & 0xfff) + 0x6000;
    }
    else {
      uVar12 = (uVar4 & 0xfff) + 0x2000;
    }
    puStack_14 = (undefined1 *)0x30;
    uStack_10 = 0x40;
    if ((*param_2 >> 0x18 < 0x30) || (0x3f < *param_2 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      uVar12 = (-(uint)(param_4 != 0) & 0xffffd312) + 0x2cee;
      uVar10 = uVar2;
    }
    else {
      puStack_14 = (undefined1 *)0x8;
      uStack_10 = 0x10;
      if ((uVar11 < 8) || (0xf < uVar11)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      uVar10 = param_1;
      if (bVar1) {
        if (((uint)param_3[9] >> 6 & 3) == 1) {
          uVar12 = (-(uint)(param_4 != 0) & 3) + 0x28b8;
          uVar10 = uVar2;
        }
        else {
          uVar12 = (-(uint)(param_4 != 0) & 3) + 0x28ea;
          uVar10 = uVar2;
        }
      }
    }
    if ((short)uVar12 != 0) {
      pvVar6 = (void *)FUN_00618b70(0x28);
      iStack_4._0_1_ = 8;
      if (pvVar6 == (void *)0x0) {
        piVar9 = (int *)0x0;
      }
      else {
        piVar9 = (int *)FUN_005fbd20(pvVar6,uVar10,uVar12,10);
      }
      iStack_4._0_1_ = 0;
      bVar1 = FUN_005fc040(piVar9);
      if (CONCAT31(extraout_var_01,bVar1) != 0) {
        FUN_005fd0f0(piVar9,puStack_24,0,0);
      }
      if (piVar9 != (int *)0x0) {
        FUN_005fbfa0(piVar9);
        FUN_00618b60((undefined *)piVar9);
      }
    }
  }
  if (((*(byte *)(param_3 + 0x14) & 4) == 0) && (param_5 != 0)) {
    if (((byte)param_3[9] & 0xc0) == 0x40) {
      uVar12 = (param_4 != 0) + 0x2d32;
    }
    else {
      uVar12 = (param_4 != 0) + 0x2d34;
    }
    if ((short)uVar12 != 0) {
      pvVar6 = (void *)FUN_00618b70(0x28);
      iStack_4._0_1_ = 9;
      if (pvVar6 == (void *)0x0) {
        piVar9 = (int *)0x0;
      }
      else {
        piVar9 = (int *)FUN_005fbd20(pvVar6,uVar2,uVar12,10);
      }
      iStack_4._0_1_ = 0;
      bVar1 = FUN_005fc040(piVar9);
      if (CONCAT31(extraout_var_02,bVar1) != 0) {
        FUN_005fd0f0(piVar9,puStack_24,0,0);
      }
      if (piVar9 != (int *)0x0) {
        FUN_005fbfa0(piVar9);
        FUN_00618b60((undefined *)piVar9);
      }
    }
  }
LAB_0042ccc2:
  puVar3 = local_1c;
  if (local_1c != (uint *)0x0) {
    FUN_005fd0f0(local_1c,puStack_24,0,0);
    FUN_005fbfa0(puVar3);
    FUN_00618b60((undefined *)puVar3);
    puStack_14 = (undefined1 *)0x30;
    uStack_10 = 0x3c;
    if (((uVar11 < 0x30) || (0x3b < uVar11)) || (param_4 != 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (((bVar1) && ((*(byte *)(param_3 + 0x2b) & 1) != 0)) && (param_5 != 0)) {
      pvVar6 = (void *)FUN_00618b70(0x28);
      iStack_4._0_1_ = 10;
      if (pvVar6 == (void *)0x0) {
        piVar9 = (int *)0x0;
      }
      else {
        piVar9 = (int *)FUN_005fbd20(pvVar6,param_1,(uVar4 & 0xfff) + 0x7000,10);
      }
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      bVar1 = FUN_005fc040(piVar9);
      if (CONCAT31(extraout_var_03,bVar1) != 0) {
        FUN_005fd0f0(piVar9,puStack_24,0,0);
      }
      if (piVar9 != (int *)0x0) {
        FUN_005fbfa0(piVar9);
        FUN_00618b60((undefined *)piVar9);
      }
    }
  }
  iStack_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return puStack_24;
}

