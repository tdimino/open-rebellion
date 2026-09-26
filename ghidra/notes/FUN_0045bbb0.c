
void __thiscall FUN_0045bbb0(void *this,uint *param_1,int param_2)

{
  bool bVar1;
  bool bVar2;
  undefined4 uVar3;
  int *piVar4;
  void *pvVar5;
  RECT *lprcSrc2;
  uint uVar6;
  undefined4 *puVar7;
  void *this_00;
  uint *puVar8;
  uint *puVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  undefined1 uVar13;
  uint local_ac;
  void *pvStack_a0;
  void *local_9c;
  undefined4 *puStack_98;
  LONG *local_94;
  int *local_90;
  int *piStack_8c;
  RECT local_88;
  uint uStack_78;
  tagRECT local_74;
  uint auStack_64 [2];
  undefined1 uStack_5c;
  undefined4 local_3c [7];
  int local_20;
  int local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630a7d;
  pvStack_c = ExceptionList;
  bVar1 = false;
  ExceptionList = &pvStack_c;
  uVar3 = FUN_006037f0(7);
  piVar4 = FUN_004f3220(*(int *)(*(int *)((int)this + 0x194) + 0x9c),param_1);
  local_90 = piVar4;
  pvVar5 = (void *)FUN_0060a860((void *)((int)this + 0x164),*param_1 & 0xffffff);
  local_9c = pvVar5;
  FUN_00526cf0(local_3c,piVar4,3);
  local_4 = 0;
  if ((*(byte *)(piVar4 + 0x14) & 8) == 0) {
    if (param_2 == 1) {
      local_ac = 2;
    }
    else if (param_2 == 2) {
      local_ac = 4;
    }
    else {
      local_ac = 1;
    }
    if ((*(byte *)(piVar4 + 0x22) & 4) != 0) {
      if (param_2 == 1) {
        local_ac = local_ac | 0x10;
      }
      else if (param_2 == 2) {
        local_ac = local_ac | 0x20;
      }
    }
    FUN_00513120((int)local_3c);
    if (local_20 != 0) {
      local_ac = local_ac | 8;
    }
  }
  else {
    local_ac = 0x40;
  }
  local_94 = (LONG *)((int)pvVar5 + 0x40);
  local_88.left = *local_94;
  local_88.top = *(LONG *)((int)pvVar5 + 0x44);
  local_88.right = *(LONG *)((int)pvVar5 + 0x48);
  local_88.bottom = *(LONG *)((int)pvVar5 + 0x4c);
  lprcSrc2 = (RECT *)FUN_0045c1f0(local_1c,(int)pvVar5);
  UnionRect(&local_74,&local_88,lprcSrc2);
  local_88.top = local_74.top;
  local_88.left = local_74.left;
  local_88.right = local_74.right;
  local_88.bottom = local_74.bottom;
  if ((local_ac & 0x40) != 0) {
    FUN_0060bd20(pvVar5,0,0x2800,uVar3);
    *(undefined4 *)((int)pvVar5 + 0x54) = 0x40;
    *(undefined4 *)((int)pvVar5 + 0x58) = 1;
    *(undefined4 *)((int)pvVar5 + 0x60) = 0x2ffff00;
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_88,0);
    goto LAB_0045c1b9;
  }
  if (((local_ac ^ *(uint *)((int)pvVar5 + 0x54)) & 7) != 0) {
    if ((local_ac & 7) == 2) {
      *(undefined4 *)((int)pvVar5 + 0x58) = 1;
      *(undefined4 *)((int)pvVar5 + 0x60) = 0x20000ff;
    }
    else {
      *(undefined4 *)((int)pvVar5 + 0x58) = 1;
      if ((local_ac & 7) == 4) {
        *(undefined4 *)((int)pvVar5 + 0x60) = 0x200ff00;
      }
      else {
        *(undefined4 *)((int)pvVar5 + 0x60) = 0x2ffff00;
      }
    }
    bVar1 = true;
  }
  if ((local_ac & 0x30) == 0x10) {
    pvStack_a0 = (void *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,1);
    if (pvStack_a0 == (void *)0x0) {
LAB_0045be1e:
      puStack_98 = (undefined4 *)0x0;
    }
    else {
      puStack_98 = FUN_005fbd20(pvStack_a0,uVar3,0x389,2);
    }
LAB_0045be20:
    local_4 = local_4 & 0xffffff00;
  }
  else {
    if ((local_ac & 0x30) == 0x20) {
      pvStack_a0 = (void *)FUN_00618b70(0x28);
      local_4 = CONCAT31(local_4._1_3_,2);
      if (pvStack_a0 == (void *)0x0) goto LAB_0045be1e;
      puStack_98 = FUN_005fbd20(pvStack_a0,uVar3,0x38a,2);
      goto LAB_0045be20;
    }
    puStack_98 = (undefined4 *)0x0;
  }
  uVar6 = local_ac ^ *(uint *)((int)pvVar5 + 0x54);
  if ((uVar6 & 0x30) != 0) {
    bVar1 = true;
  }
  if ((uVar6 & 8) != 0) {
    puVar7 = (undefined4 *)FUN_0060a860((void *)((int)this + 0x174),0x10000);
    if ((local_ac & 8) == 0) {
      if (puVar7 != (undefined4 *)0x0) {
        puVar8 = FUN_0042d170(puVar7,&local_94);
        local_4._0_1_ = 5;
        piVar4 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x194) + 0x9c),puVar8);
        local_4._0_1_ = 0;
        FUN_00619730();
        piStack_8c = piVar4;
        if (piVar4 == (int *)0x0) {
LAB_0045bfe8:
          bVar2 = false;
          bVar1 = false;
          uVar6 = 0;
        }
        else {
          if (piVar4[7] == 0) {
            pvVar5 = (void *)0x0;
          }
          else {
            pvVar5 = *(void **)(piVar4[7] + 0x1c);
          }
          if (pvVar5 == (void *)0x0) goto LAB_0045bfe8;
          puVar8 = FUN_004025b0(*(void **)((int)this + 0x144),(uint *)&pvStack_a0);
          local_4._0_1_ = 6;
          puVar9 = FUN_004025b0(pvVar5,&uStack_78);
          bVar2 = true;
          bVar1 = true;
          uVar6 = (uint)(*puVar9 == *puVar8);
        }
        piVar4 = piStack_8c;
        if (bVar2) {
          FUN_00619730();
        }
        local_4 = 0;
        if (bVar1) {
          FUN_00619730();
        }
        if (piVar4 == (int *)0x0) {
          uVar10 = 0;
        }
        else {
          uVar10 = (uint)~piVar4[0x14] >> 3 & 1;
        }
        pvVar5 = local_9c;
        piVar4 = local_90;
        if (((uVar10 & uVar6) == 0) &&
           (iVar11 = FUN_005f5ac0((void *)((int)this + 0x174),(int)puVar7), pvVar5 = local_9c,
           piVar4 = local_90, iVar11 != 0)) {
          (**(code **)*puVar7)(1);
          pvVar5 = local_9c;
          piVar4 = local_90;
        }
      }
    }
    else {
      this_00 = (void *)FUN_0052bed0((int)local_3c);
      if (puVar7 == (undefined4 *)0x0) {
        pvVar5 = (void *)FUN_00618b70(0x74);
        local_4._0_1_ = 3;
        pvStack_a0 = pvVar5;
        if (pvVar5 == (void *)0x0) {
          puVar7 = (undefined4 *)0x0;
        }
        else {
          puVar8 = FUN_004025b0(this_00,(uint *)&piStack_8c);
          local_4 = CONCAT31(local_4._1_3_,4);
          puVar7 = FUN_00442130(pvVar5,0x10000,puVar8);
        }
        local_4 = 3;
        if (pvVar5 != (void *)0x0) {
          FUN_00619730();
        }
        local_4 = local_4 & 0xffffff00;
        FUN_005f59f0((void *)((int)this + 0x174),(int)puVar7);
        FUN_0060bd20(puVar7,0x388,0,uVar3);
        puVar7[0x15] = 1;
        puVar7[0xf] = 6;
        pvVar5 = local_9c;
      }
      puVar7[0x10] = *local_94;
      puVar7[0x11] = local_94[1];
      puVar7[0x12] = local_94[2];
      puVar7[0x13] = local_94[3];
      piVar4 = local_90;
    }
    bVar1 = true;
  }
  iVar11 = FUN_0042d860(*(void **)((int)this + 0x194),piVar4);
  if (iVar11 != *(int *)((int)pvVar5 + 0x70)) {
    bVar1 = true;
    *(int *)((int)pvVar5 + 0x70) = iVar11;
  }
  if (bVar1) {
    uVar13 = 2;
    iVar11 = FUN_00509610((int)piVar4);
    uVar6 = FUN_0045c970(iVar11);
    FUN_005fbd20(auStack_64,uVar3,uVar6,uVar13);
    local_4._0_1_ = 7;
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 8;
    pvStack_a0 = pvVar5;
    if (pvVar5 == (void *)0x0) {
      puVar8 = (uint *)0x0;
    }
    else {
      puVar8 = auStack_64;
      iVar11 = FUN_005fc0f0((int *)auStack_64);
      iVar11 = iVar11 + 4;
      iVar12 = FUN_005fc0e0((int *)auStack_64);
      puVar8 = FUN_005fbda0(pvVar5,iVar12,iVar11,puVar8);
    }
    local_4 = CONCAT31(local_4._1_3_,7);
    *(undefined1 *)((int)this + 0x158) = uStack_5c;
    uVar13 = uStack_5c;
    iVar11 = FUN_005fc0f0((int *)puVar8);
    uVar6 = FUN_005fc0e0((int *)puVar8);
    FUN_005fcfe0(puVar8,0,0,uVar6,iVar11,uVar13);
    FUN_005fcc30(auStack_64,puVar8,0,0,0,0,(undefined4 *)0x0,0);
    if (puStack_98 != (undefined4 *)0x0) {
      FUN_005fd0f0(puStack_98,puVar8,0,0);
    }
    piVar4 = *(int **)((int)local_9c + 0x70);
    if (piVar4 != (int *)0x0) {
      iVar11 = FUN_005fc0f0((int *)puVar8);
      iVar12 = FUN_005fc0f0(piVar4);
      FUN_005fd0f0(piVar4,puVar8,0,iVar11 - iVar12);
    }
    pvVar5 = local_9c;
    FUN_0060be60(local_9c,0,(int)puVar8);
    *(uint *)((int)pvVar5 + 0x54) = local_ac;
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_88,0);
    local_4 = local_4 & 0xffffff00;
    FUN_005fbfa0(auStack_64);
  }
  puVar7 = puStack_98;
  if (puStack_98 != (undefined4 *)0x0) {
    FUN_005fbfa0(puStack_98);
    FUN_00618b60((undefined *)puVar7);
  }
LAB_0045c1b9:
  local_4 = 0xffffffff;
  FUN_00526df0(local_3c);
  ExceptionList = pvStack_c;
  return;
}

