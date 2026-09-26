
void __thiscall FUN_00464e10(void *this,void *param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  int iVar5;
  uint *puVar6;
  int *piVar7;
  HDC pHVar8;
  HDC hdc;
  void *pvVar9;
  void *pvVar10;
  undefined *puVar11;
  char *pcVar12;
  uint uVar13;
  HGDIOBJ pvVar14;
  undefined4 uVar15;
  int *this_00;
  uint uVar16;
  undefined1 uVar17;
  int iVar18;
  undefined4 uVar19;
  HDC local_bc;
  int *piStack_b8;
  void *pvStack_b4;
  int iStack_b0;
  undefined4 local_ac;
  RECT RStack_a8;
  undefined1 local_98 [4];
  uint uStack_94;
  uint uStack_90;
  undefined4 auStack_8c [7];
  int iStack_70;
  undefined4 auStack_6c [7];
  int iStack_50;
  undefined4 auStack_4c [8];
  undefined4 auStack_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006317a6;
  local_c = ExceptionList;
  pvVar9 = *(void **)((int)param_1 + 8);
  iVar18 = *(int *)((int)this + 0x144);
  bVar4 = false;
  bVar3 = false;
  if (*(int *)(iVar18 + 0x94) == 0) {
    iVar5 = 0;
  }
  else {
    iVar5 = *(int *)(*(int *)(iVar18 + 0x94) + 0x24);
  }
  local_ac = 4;
  iVar1 = *(int *)(*(int *)((int)this + 0x148) + 0x94);
  uVar16 = (iVar5 != 1) + 1;
  if (iVar1 == 0) {
    iVar5 = 0;
  }
  else {
    iVar5 = *(int *)(iVar1 + 0x24);
  }
  if (iVar5 == 2) {
    if (*(int *)(iVar18 + 0x94) == 0) {
      iVar18 = 0;
    }
    else {
      iVar18 = *(int *)(*(int *)(iVar18 + 0x94) + 0x24);
    }
    if (iVar18 == 1) {
      iVar18 = 2;
    }
    else {
      iVar18 = 3;
    }
    ExceptionList = &local_c;
    FUN_006075e0(this,iVar18);
    FUN_005f5b20(param_1);
    pHVar8 = GetDC(*(HWND *)((int)this + 0x18));
    hdc = CreateCompatibleDC(pHVar8);
    local_bc = hdc;
    ReleaseDC(*(HWND *)((int)this + 0x18),pHVar8);
    piVar7 = FUN_004f31b0(*(int *)(*(int *)((int)this + 0x154) + 0x9c));
    iStack_b0 = 0;
    while (piStack_b8 = piVar7, piVar7 != (int *)0x0) {
      FUN_005040c0(auStack_4c,piVar7,3,uVar16);
      local_4 = 0;
      iVar18 = FUN_00513180((int)auStack_4c);
      if (iVar18 != 0) {
        pvVar9 = (void *)FUN_00618b70(0x74);
        local_4._0_1_ = 1;
        pvStack_b4 = pvVar9;
        if (pvVar9 == (void *)0x0) {
          pvVar9 = (void *)0x0;
        }
        else {
          puVar6 = &uStack_90;
          iVar18 = FUN_00513120((int)auStack_4c);
          pvVar10 = (void *)FUN_0052bed0(iVar18);
          puVar6 = FUN_004025b0(pvVar10,puVar6);
          bVar3 = true;
          iStack_b0 = iStack_b0 + 1;
          local_4 = CONCAT31(local_4._1_3_,2);
          pvVar9 = FUN_00442130(pvVar9,iStack_b0,puVar6);
        }
        local_4 = 1;
        if (bVar3) {
          bVar3 = false;
          FUN_00619730();
        }
        local_4 = local_4 & 0xffffff00;
        *(undefined4 *)((int)pvVar9 + 0x3c) = 4;
        puVar11 = FUN_004f62d0((int)piVar7);
        pcVar12 = (char *)FUN_00583c40((int)puVar11);
        FUN_005f35e0((void *)((int)pvVar9 + 0x14),pcVar12);
        pvVar10 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 3;
        pvStack_b4 = pvVar10;
        if (pvVar10 == (void *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          pHVar8 = hdc;
          puVar6 = (uint *)FUN_006073d0(this);
          piVar7 = FUN_005fbea0(pvVar10,0x15e,0x14,puVar6,pHVar8);
        }
        uVar17 = 0;
        local_4 = (uint)local_4._1_3_ << 8;
        iVar18 = FUN_005fc0f0(piVar7);
        uVar13 = FUN_005fc0e0(piVar7);
        FUN_005fcfe0(piVar7,0,0,uVar13,iVar18,uVar17);
        pvVar14 = (HGDIOBJ)FUN_005fc130((int)piVar7);
        pvStack_b4 = SelectObject(hdc,pvVar14);
        FUN_00465540(piStack_b8,uVar16,hdc);
        SelectObject(hdc,pvStack_b4);
        FUN_0060be60(pvVar9,(int)piVar7,0);
        FUN_005f59f0(param_1,(int)pvVar9);
        piVar7 = piStack_b8;
      }
      FUN_004ffef0(auStack_6c,piVar7,3,uVar16);
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_00513120((int)auStack_6c);
      while (iStack_50 != 0) {
        piVar7 = (int *)FUN_0052bed0((int)auStack_6c);
        if (((uint)piVar7[9] >> 6 & 3) == uVar16) {
          FUN_00502db0(auStack_8c,piVar7,3);
          local_4 = CONCAT31(local_4._1_3_,5);
          bVar2 = false;
          FUN_00513120((int)auStack_8c);
          while (iStack_70 != 0) {
            uVar19 = 3;
            uVar15 = FUN_0052bed0((int)auStack_8c);
            FUN_00504040(auStack_2c,uVar15,uVar19);
            local_4._0_1_ = 6;
            iVar18 = FUN_00513180((int)auStack_2c);
            if (iVar18 != 0) {
              bVar2 = true;
            }
            local_4 = CONCAT31(local_4._1_3_,5);
            FUN_00504140(auStack_2c);
            FUN_005130d0((int)auStack_8c);
          }
          if (bVar2) {
            pvVar9 = (void *)FUN_00618b70(0x74);
            local_4._0_1_ = 7;
            pvStack_b4 = pvVar9;
            if (pvVar9 == (void *)0x0) {
              pvVar9 = (void *)0x0;
            }
            else {
              puVar6 = FUN_004025b0(piVar7,&uStack_94);
              bVar4 = true;
              iStack_b0 = iStack_b0 + 1;
              local_4 = CONCAT31(local_4._1_3_,8);
              pvVar9 = FUN_00442130(pvVar9,iStack_b0,puVar6);
            }
            local_4 = 7;
            if (bVar4) {
              bVar4 = false;
              FUN_00619730();
            }
            local_4._0_1_ = 5;
            *(undefined4 *)((int)pvVar9 + 0x3c) = 4;
            puVar11 = FUN_004f62d0((int)piVar7);
            pcVar12 = (char *)FUN_00583c40((int)puVar11);
            FUN_005f35e0((void *)((int)pvVar9 + 0x14),pcVar12);
            pvVar10 = (void *)FUN_00618b70(0x28);
            local_4._0_1_ = 9;
            pvStack_b4 = pvVar10;
            if (pvVar10 == (void *)0x0) {
              this_00 = (int *)0x0;
            }
            else {
              pHVar8 = hdc;
              puVar6 = (uint *)FUN_006073d0(this);
              this_00 = FUN_005fbea0(pvVar10,0x15e,0x14,puVar6,pHVar8);
            }
            uVar17 = 0;
            local_4 = CONCAT31(local_4._1_3_,5);
            iVar18 = FUN_005fc0f0(this_00);
            uVar13 = FUN_005fc0e0(this_00);
            FUN_005fcfe0(this_00,0,0,uVar13,iVar18,uVar17);
            pvVar14 = (HGDIOBJ)FUN_005fc130((int)this_00);
            pvVar14 = SelectObject(hdc,pvVar14);
            FUN_00465540(piVar7,uVar16,local_bc);
            SelectObject(local_bc,pvVar14);
            FUN_0060be60(pvVar9,(int)this_00,0);
            FUN_005f59f0(param_1,(int)pvVar9);
            hdc = local_bc;
          }
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_00502eb0(auStack_8c);
        }
        FUN_005130d0((int)auStack_6c);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_004fff70(auStack_6c);
      local_4 = 0xffffffff;
      FUN_00504140(auStack_4c);
      RStack_a8.left = 0x90;
      RStack_a8.top = 0x98;
      local_4 = 10;
      piVar7 = (int *)FUN_004f6010(piStack_b8,(uint *)&RStack_a8,1);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    DeleteDC(hdc);
  }
  else {
    if (iVar5 != 3) {
      return;
    }
    if (*(int *)(iVar18 + 0x94) == 0) {
      iVar18 = 0;
    }
    else {
      iVar18 = *(int *)(*(int *)(iVar18 + 0x94) + 0x24);
    }
    if (iVar18 == 1) {
      iVar18 = 4;
    }
    else {
      iVar18 = 5;
    }
    ExceptionList = &local_c;
    FUN_006075e0(this,iVar18);
    FUN_004ece30(&local_bc);
    local_ac = 10;
    local_4 = 0xb;
    for (; pvVar9 != (void *)0x0; pvVar9 = *(void **)((int)pvVar9 + 8)) {
      puVar6 = FUN_0042d170(pvVar9,local_98);
      local_4._0_1_ = 0xc;
      piVar7 = FUN_004f2ec0(*(int *)(*(int *)((int)this + 0x154) + 0x9c),puVar6);
      local_4 = CONCAT31(local_4._1_3_,0xb);
      FUN_00619730();
      *(undefined4 *)((int)pvVar9 + 0x6c) = 0;
      if (((piVar7 != (int *)0x0) && ((*(byte *)(piVar7 + 0x14) & 2) != 0)) &&
         (*(uint *)((int)pvVar9 + 0x54) == uVar16)) {
        FUN_00465bb0(piVar7);
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  uVar16 = *(uint *)(*(int *)((int)this + 0x150) + 0x2c);
  iVar18 = *(int *)(*(int *)((int)this + 0x150) + 0x28);
  puVar6 = (uint *)FUN_006073d0(this);
  FUN_00608300(*(void **)((int)this + 0x150),puVar6,iVar18,uVar16);
  FUN_00608280(*(void **)((int)this + 0x150),param_1);
  (**(code **)(**(int **)((int)this + 0x150) + 0x18))(local_ac);
  FUN_0060a280(*(void **)((int)this + 0x150));
  iVar18 = *(int *)(*(int *)((int)this + 0x144) + 0x94);
  if (iVar18 == 0) {
    uVar16 = 0;
  }
  else {
    uVar16 = *(uint *)(iVar18 + 0x24);
  }
  iVar18 = FUN_00604500((void *)(*(int *)((int)this + 0x144) + 0x6c),uVar16);
  if (iVar18 != 0) {
    FUN_00601aa0(*(void **)((int)this + 0x158),*(char **)(iVar18 + 0x84));
  }
  iVar18 = *(int *)(*(int *)((int)this + 0x148) + 0x94);
  if (iVar18 == 0) {
    iVar18 = 0;
  }
  else {
    iVar18 = *(int *)(iVar18 + 0x24);
  }
  if (iVar18 == 2) {
    iVar18 = 0x77;
  }
  else {
    iVar18 = 0x73;
  }
  FUN_00601b30(*(void **)((int)this + 0x158),0x28,iVar18);
  iVar18 = *(int *)((int)this + 0x158);
  RStack_a8.left = *(LONG *)(iVar18 + 0x14);
  RStack_a8.top = *(LONG *)(iVar18 + 0x18);
  RStack_a8.right = *(LONG *)(iVar18 + 0x1c);
  RStack_a8.bottom = *(LONG *)(iVar18 + 0x20);
  InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_a8,0);
  ExceptionList = local_c;
  return;
}

