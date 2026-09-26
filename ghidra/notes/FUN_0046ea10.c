
void __thiscall FUN_0046ea10(void *this,uint param_1)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  uint *puVar4;
  HDC pHVar5;
  HDC hdc;
  int *this_00;
  int iVar6;
  void *pvVar7;
  undefined *puVar8;
  char *pcVar9;
  void *pvVar10;
  int *piVar11;
  uint uVar12;
  HGDIOBJ pvVar13;
  undefined4 uVar14;
  int *this_01;
  undefined1 uVar15;
  undefined4 uVar16;
  uint local_b8;
  int iStack_ac;
  RECT RStack_a4;
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
  uint uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00632380;
  local_c = ExceptionList;
  bVar3 = false;
  bVar2 = false;
  if (*(uint *)((int)this + 0x158) != param_1) {
    if (param_1 == 1) {
      local_b8 = param_1;
    }
    else {
      local_b8 = 2;
    }
    ExceptionList = &local_c;
    FUN_006075e0(this,local_b8);
    uVar12 = *(uint *)(*(int *)((int)this + 0x14c) + 0x2c);
    iVar6 = *(int *)(*(int *)((int)this + 0x14c) + 0x28);
    puVar4 = (uint *)FUN_006073d0(this);
    FUN_00608300(*(void **)((int)this + 0x14c),puVar4,iVar6,uVar12);
    FUN_005f5b20(*(void **)((int)this + 0x178));
    pHVar5 = GetDC(*(HWND *)((int)this + 0x18));
    hdc = CreateCompatibleDC(pHVar5);
    ReleaseDC(*(HWND *)((int)this + 0x18),pHVar5);
    this_00 = FUN_004f31b0(*(int *)(*(int *)((int)this + 0x150) + 0x9c));
    iStack_ac = 0;
    while (this_00 != (int *)0x0) {
      FUN_00504cc0(auStack_2c,this_00,3,local_b8);
      uStack_4 = 0;
      iVar6 = FUN_00513180((int)auStack_2c);
      if (iVar6 != 0) {
        pvVar7 = (void *)FUN_00618b70(0x74);
        uStack_4._0_1_ = 1;
        if (pvVar7 == (void *)0x0) {
          pvVar7 = (void *)0x0;
        }
        else {
          puVar4 = FUN_004025b0(this_00,&uStack_94);
          bVar2 = true;
          iStack_ac = iStack_ac + 1;
          uStack_4 = CONCAT31(uStack_4._1_3_,2);
          pvVar7 = FUN_00442130(pvVar7,iStack_ac,puVar4);
        }
        uStack_4 = 1;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        uStack_4 = uStack_4 & 0xffffff00;
        *(undefined4 *)((int)pvVar7 + 0x3c) = 4;
        puVar8 = FUN_004f62d0((int)this_00);
        pcVar9 = (char *)FUN_00583c40((int)puVar8);
        FUN_005f35e0((void *)((int)pvVar7 + 0x14),pcVar9);
        pvVar10 = (void *)FUN_00618b70(0x28);
        uStack_4._0_1_ = 3;
        if (pvVar10 == (void *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          pHVar5 = hdc;
          puVar4 = (uint *)FUN_006073d0(this);
          piVar11 = FUN_005fbea0(pvVar10,0x14a,0x19,puVar4,pHVar5);
        }
        uVar15 = 0;
        uStack_4 = (uint)uStack_4._1_3_ << 8;
        iVar6 = FUN_005fc0f0(piVar11);
        uVar12 = FUN_005fc0e0(piVar11);
        FUN_005fcfe0(piVar11,0,0,uVar12,iVar6,uVar15);
        pvVar13 = (HGDIOBJ)FUN_005fc130((int)piVar11);
        pvVar13 = SelectObject(hdc,pvVar13);
        FUN_0046d8d0(this_00,local_b8,hdc);
        SelectObject(hdc,pvVar13);
        FUN_0060be60(pvVar7,(int)piVar11,0);
        FUN_005f59f0(*(void **)((int)this + 0x178),(int)pvVar7);
      }
      FUN_004ffef0(auStack_8c,this_00,3,local_b8);
      uStack_4 = CONCAT31(uStack_4._1_3_,4);
      FUN_00513120((int)auStack_8c);
      while (iStack_70 != 0) {
        piVar11 = (int *)FUN_0052bed0((int)auStack_8c);
        if (((uint)piVar11[9] >> 6 & 3) == local_b8) {
          FUN_00502db0(auStack_6c,piVar11,3);
          uStack_4 = CONCAT31(uStack_4._1_3_,5);
          bVar1 = false;
          FUN_00513120((int)auStack_6c);
          while (iStack_50 != 0) {
            uVar16 = 3;
            uVar14 = FUN_0052bed0((int)auStack_6c);
            FUN_00504c40(auStack_4c,uVar14,uVar16);
            uStack_4._0_1_ = 6;
            iVar6 = FUN_00513180((int)auStack_4c);
            if (iVar6 != 0) {
              bVar1 = true;
            }
            uStack_4 = CONCAT31(uStack_4._1_3_,5);
            FUN_00504d40(auStack_4c);
            FUN_005130d0((int)auStack_6c);
          }
          if (bVar1) {
            pvVar7 = (void *)FUN_00618b70(0x74);
            uStack_4._0_1_ = 7;
            if (pvVar7 == (void *)0x0) {
              pvVar7 = (void *)0x0;
            }
            else {
              puVar4 = FUN_004025b0(piVar11,&uStack_90);
              bVar3 = true;
              iStack_ac = iStack_ac + 1;
              uStack_4 = CONCAT31(uStack_4._1_3_,8);
              pvVar7 = FUN_00442130(pvVar7,iStack_ac,puVar4);
            }
            uStack_4 = 7;
            if (bVar3) {
              bVar3 = false;
              FUN_00619730();
            }
            uStack_4._0_1_ = 5;
            *(undefined4 *)((int)pvVar7 + 0x3c) = 4;
            puVar8 = FUN_004f62d0((int)piVar11);
            pcVar9 = (char *)FUN_00583c40((int)puVar8);
            FUN_005f35e0((void *)((int)pvVar7 + 0x14),pcVar9);
            pvVar10 = (void *)FUN_00618b70(0x28);
            uStack_4._0_1_ = 9;
            if (pvVar10 == (void *)0x0) {
              this_01 = (int *)0x0;
            }
            else {
              pHVar5 = hdc;
              puVar4 = (uint *)FUN_006073d0(this);
              this_01 = FUN_005fbea0(pvVar10,0x14a,0x19,puVar4,pHVar5);
            }
            uVar15 = 0;
            uStack_4 = CONCAT31(uStack_4._1_3_,5);
            iVar6 = FUN_005fc0f0(this_01);
            uVar12 = FUN_005fc0e0(this_01);
            FUN_005fcfe0(this_01,0,0,uVar12,iVar6,uVar15);
            pvVar13 = (HGDIOBJ)FUN_005fc130((int)this_01);
            pvVar13 = SelectObject(hdc,pvVar13);
            FUN_0046d8d0(piVar11,local_b8,hdc);
            SelectObject(hdc,pvVar13);
            FUN_0060be60(pvVar7,(int)this_01,0);
            FUN_005f59f0(*(void **)((int)this + 0x178),(int)pvVar7);
          }
          uStack_4 = CONCAT31(uStack_4._1_3_,4);
          FUN_00502eb0(auStack_6c);
        }
        FUN_005130d0((int)auStack_8c);
      }
      RStack_a4.left = 0x90;
      RStack_a4.top = 0x98;
      uStack_4._1_3_ = (uint3)(uStack_4 >> 8);
      uStack_4._0_1_ = 10;
      this_00 = (int *)FUN_004f6010(this_00,(uint *)&RStack_a4,1);
      uStack_4._0_1_ = 4;
      FUN_00619730();
      uStack_4 = (uint)uStack_4._1_3_ << 8;
      FUN_004fff70(auStack_8c);
      uStack_4 = 0xffffffff;
      FUN_00504d40(auStack_2c);
    }
    DeleteDC(hdc);
    *(uint *)((int)this + 0x158) = param_1;
    FUN_0060a280(*(void **)((int)this + 0x14c));
    iVar6 = FUN_00604500((void *)(*(int *)((int)this + 0x144) + 0x6c),param_1);
    FUN_00601aa0(*(void **)((int)this + 0x154),*(char **)(iVar6 + 0x84));
    iVar6 = *(int *)((int)this + 0x154);
    RStack_a4.left = *(LONG *)(iVar6 + 0x14);
    RStack_a4.top = *(LONG *)(iVar6 + 0x18);
    RStack_a4.right = *(LONG *)(iVar6 + 0x1c);
    RStack_a4.bottom = *(LONG *)(iVar6 + 0x20);
    InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_a4,0);
  }
  ExceptionList = local_c;
  return;
}

