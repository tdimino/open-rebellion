
void __fastcall FUN_004a5c00(void *param_1)

{
  bool bVar1;
  int *piVar2;
  char *pcVar3;
  uint *puVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined3 extraout_var;
  int *piVar7;
  HDC hdc;
  HDC hdc_00;
  HGDIOBJ pvVar8;
  HGDIOBJ pvVar9;
  void *pvVar10;
  LPCSTR pCVar11;
  HPALETTE pHVar12;
  int iVar13;
  uint uVar14;
  tagRECT *ptVar15;
  HWND pHVar16;
  UINT UVar17;
  int *local_98;
  uint local_94;
  HPALETTE local_90;
  int local_8c;
  int *piStack_88;
  tagRECT tStack_84;
  int iStack_74;
  int aiStack_70 [2];
  undefined1 auStack_68 [8];
  undefined4 auStack_60 [20];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00637dbd;
  pvStack_c = ExceptionList;
  local_98 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_94 = FUN_005fefd0(7);
  local_8c = 0;
  bVar1 = false;
  local_90 = (HPALETTE)0x0;
  piVar2 = (int *)(**(code **)(**(int **)((int)param_1 + 0x188) + 8))();
  FUN_005f5b20(*(void **)((int)param_1 + 0x18c));
  *(undefined4 *)((int)param_1 + 0x198) = 0;
  *(undefined4 *)((int)param_1 + 0x19c) = 0;
  for (; piVar2 != (int *)0x0; piVar2 = (int *)(**(code **)(*piVar2 + 0xc))()) {
    if ((*(byte *)(piVar2 + 0xf) & 1) != 0) {
      if ((*(byte *)(piVar2 + 0x1a) & 8) != 0) {
        local_8c = 1;
      }
      local_90 = (HPALETTE)((int)local_90 + 1);
      local_98 = piVar2;
    }
  }
  piVar2 = (int *)(**(code **)(**(int **)((int)param_1 + 0x188) + 8))();
  piVar7 = local_98;
  while ((piVar2 != (int *)0x0 && (local_8c != 0))) {
    local_98 = piVar7;
    if (((piVar2[0xf] & 1U) != 0) && (local_98 = piVar2, (*(byte *)(piVar2 + 0x1a) & 8) == 0)) {
      local_90 = (HPALETTE)((int)local_90 + -1);
      piVar2[0xf] = piVar2[0xf] & 0xfffffffe;
      bVar1 = true;
      local_98 = piVar7;
    }
    piVar2 = (int *)(**(code **)(*piVar2 + 0xc))();
    piVar7 = local_98;
  }
  if (bVar1) {
    FUN_0060a280(*(void **)((int)param_1 + 0x160));
  }
  pHVar12 = local_90;
  if ((int)local_90 < 2) {
    if (piVar7 == (int *)0x0) {
      FUN_00601aa0(*(void **)((int)param_1 + 400),(char *)0x0);
      pHVar16 = *(HWND *)((int)param_1 + 0x18);
    }
    else {
      if ((*(byte *)(piVar7 + 0x1a) & 4) == 0) {
        if ((*(byte *)((int)param_1 + 0x14c) & 8) == 0) {
          uVar14 = *(uint *)((int)param_1 + 0x14c) | 8;
          goto LAB_004a5d58;
        }
      }
      else if ((*(byte *)((int)param_1 + 0x14c) & 8) != 0) {
        uVar14 = *(uint *)((int)param_1 + 0x14c) & 0xfffffff7;
LAB_004a5d58:
        *(uint *)((int)param_1 + 0x14c) = uVar14;
      }
      pcVar3 = (char *)FUN_00583c40((int)(piVar7 + 5));
      FUN_00601aa0(*(void **)((int)param_1 + 400),pcVar3);
      pHVar16 = *(HWND *)((int)param_1 + 0x18);
    }
    FUN_00601dc0(*(void **)((int)param_1 + 400),pHVar16,0);
  }
  else {
    if ((*(byte *)((int)param_1 + 0x14c) & 8) != 0) {
      *(uint *)((int)param_1 + 0x14c) = *(uint *)((int)param_1 + 0x14c) & 0xfffffff7;
    }
    FUN_00601aa0(*(void **)((int)param_1 + 400),(char *)0x0);
    FUN_00601dc0(*(void **)((int)param_1 + 400),*(HWND *)((int)param_1 + 0x18),0);
  }
  if (piVar7 == (int *)0x0) {
    puVar4 = *(uint **)((int)param_1 + 0x168);
    uVar14 = 0x2a;
    iVar13 = 100;
    pvVar10 = *(void **)((int)param_1 + 0xa4);
  }
  else if ((*(byte *)(piVar7 + 0x1a) & 8) == 0) {
    puVar4 = *(uint **)((int)param_1 + 0x168);
    pvVar10 = *(void **)((int)param_1 + 0x184);
    uVar14 = 0xd;
    iVar13 = 3;
  }
  else {
    puVar4 = *(uint **)((int)param_1 + 0x168);
    pvVar10 = *(void **)((int)param_1 + 0x180);
    uVar14 = 0xd;
    iVar13 = 3;
  }
  FUN_005fcc30(pvVar10,puVar4,0,0,iVar13,uVar14,(undefined4 *)0x0,0);
  InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)&lpRect_006a8818,0);
  if ((*(byte *)((int)param_1 + 0x14c) & 8) == 0) {
    if (pHVar12 != (HPALETTE)0x0) {
      uVar14 = -(uint)(*(int *)((int)param_1 + 0x170) != 1) & 0x32;
      if (local_8c == 0) {
        uVar14 = 0x32 - uVar14;
      }
      piStack_88 = (int *)FUN_00618b70(0x28);
      uStack_4 = 1;
      if (piStack_88 == (int *)0x0) {
        piStack_88 = (int *)0x0;
      }
      else {
        piStack_88 = (int *)FUN_005fbd20(piStack_88,local_94,uVar14 + 0x28b9,2);
      }
      piVar2 = piStack_88;
      uStack_4 = 0xffffffff;
      iVar13 = FUN_005fc0e0(*(int **)((int)param_1 + 0x168));
      iVar5 = FUN_005fc0e0(piVar2);
      iVar13 = (iVar13 - iVar5) / 2;
      piVar7 = FUN_004f36a0(*(int *)(*(int *)((int)param_1 + 0x148) + 0x9c),(uint *)(piVar7 + 0x1b))
      ;
      if (piVar7 != (int *)0x0) {
        if ((*(byte *)(piVar7 + 0x14) & 0x10) != 0) {
          tStack_84.left = FUN_00618b70(0x28);
          uStack_4 = 2;
          if ((void *)tStack_84.left == (void *)0x0) {
            puVar6 = (undefined4 *)0x0;
          }
          else {
            puVar6 = (undefined4 *)FUN_005fbd20((void *)tStack_84.left,local_94,uVar14 + 0x28ba,2);
          }
          uStack_4 = 0xffffffff;
          FUN_005fd0f0(puVar6,*(uint **)((int)param_1 + 0x168),iVar13,0);
          piVar2 = piStack_88;
          if (puVar6 != (undefined4 *)0x0) {
            FUN_005fbfa0(puVar6);
            FUN_00618b60((undefined *)puVar6);
            piVar2 = piStack_88;
          }
        }
        if ((piVar7[0x14] & 0x200U) != 0) {
          tStack_84.left = FUN_00618b70(0x28);
          uStack_4 = 3;
          if ((void *)tStack_84.left == (void *)0x0) {
            puVar6 = (undefined4 *)0x0;
          }
          else {
            puVar6 = (undefined4 *)FUN_005fbd20((void *)tStack_84.left,local_94,uVar14 + 0x28bb,2);
          }
          uStack_4 = 0xffffffff;
          FUN_005fd0f0(puVar6,*(uint **)((int)param_1 + 0x168),iVar13,0);
          if (puVar6 != (undefined4 *)0x0) {
            FUN_005fbfa0(puVar6);
            FUN_00618b60((undefined *)puVar6);
          }
        }
      }
      pHVar12 = local_90;
      if (piVar2 != (int *)0x0) {
        bVar1 = FUN_005fc040(piVar2);
        if (CONCAT31(extraout_var,bVar1) != 0) {
          FUN_005fd0f0(piVar2,*(uint **)((int)param_1 + 0x168),iVar13,0);
        }
        pHVar12 = local_90;
        if (piVar2 != (int *)0x0) {
          FUN_005fbfa0(piVar2);
          FUN_00618b60((undefined *)piVar2);
          pHVar12 = local_90;
        }
      }
    }
  }
  else if ((piVar7 != (int *)0x0) &&
          (piVar2 = FUN_004f2d10(*(int *)((int)param_1 + 0x174),(uint *)(piVar7 + 0x1b)),
          piVar2 != (int *)0x0)) {
    puVar4 = FUN_004025b0(piVar2,&local_94);
    uStack_4 = 0;
    puVar4 = FUN_0042c3b0(*(undefined4 *)((int)param_1 + 0x1b0),puVar4,piVar2,1,1);
    uStack_4 = 0xffffffff;
    FUN_00619730();
    FUN_005fd0f0(puVar4,*(uint **)((int)param_1 + 0x168),0,0);
    if (puVar4 != (uint *)0x0) {
      FUN_005fbfa0(puVar4);
      FUN_00618b60((undefined *)puVar4);
    }
  }
  if (local_8c == 0) {
    if ((int)pHVar12 < 1) {
      (**(code **)(**(int **)((int)param_1 + 0x164) + 8))();
      *(uint *)((int)param_1 + 0x14c) = *(uint *)((int)param_1 + 0x14c) & 0xffffffbb;
      goto LAB_004a616b;
    }
    (**(code **)(**(int **)((int)param_1 + 0x164) + 4))(5);
    if ((*(byte *)((int)param_1 + 0x14c) & 0x40) == 0) {
      uVar14 = *(uint *)((int)param_1 + 0x14c);
      *(uint *)((int)param_1 + 0x14c) = uVar14 | 0x40;
      *(uint *)((int)param_1 + 0x14c) = uVar14 & 0xfffffffb | 0x40;
      FUN_00608300(*(void **)((int)param_1 + 0x164),*(uint **)((int)param_1 + 0x184),4,0x62);
    }
  }
  else {
    (**(code **)(**(int **)((int)param_1 + 0x164) + 4))(5);
    if ((*(byte *)((int)param_1 + 0x14c) & 4) == 0) {
      uVar14 = *(uint *)((int)param_1 + 0x14c);
      *(uint *)((int)param_1 + 0x14c) = uVar14 | 4;
      *(uint *)((int)param_1 + 0x14c) = uVar14 & 0xffffffbf | 4;
      FUN_00608300(*(void **)((int)param_1 + 0x164),*(uint **)((int)param_1 + 0x180),4,0x62);
      FUN_00601c90(*(void **)((int)param_1 + 400),
                   (-(uint)(*(int *)((int)param_1 + 0x170) != 1) & 0xfe01) + 0x20000ff,(HWND)0x0);
      goto LAB_004a616b;
    }
  }
  FUN_00601c90(*(void **)((int)param_1 + 400),
               (-(uint)(*(int *)((int)param_1 + 0x170) != 1) & 0xfe01) + 0x20000ff,(HWND)0x0);
LAB_004a616b:
  FUN_004a6980((int)param_1);
  for (piVar2 = (int *)(**(code **)(**(int **)((int)param_1 + 0x188) + 8))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 0xc))()) {
    if (((*(byte *)(piVar2 + 0xf) & 1) != 0) &&
       (piVar7 = FUN_004f2d10(*(int *)((int)param_1 + 0x174),(uint *)(piVar2 + 0x1b)),
       piVar7 != (int *)0x0)) {
      FUN_004a6be0(param_1,piVar7);
    }
  }
  if (*(void **)((int)param_1 + 0x164) != (void *)0x0) {
    FUN_0060a280(*(void **)((int)param_1 + 0x164));
  }
  if (((*(int *)((int)param_1 + 0x194) == 0x67) || (*(int *)((int)param_1 + 0x194) == 0x68)) &&
     (pHVar12 != (HPALETTE)0x0)) {
    tStack_84.left = 1;
    tStack_84.top = 2;
    tStack_84.right = 0x7b;
    tStack_84.bottom = 0x2f;
    hdc = GetDC(*(HWND *)((int)param_1 + 0x18));
    hdc_00 = CreateCompatibleDC(hdc);
    ReleaseDC(*(HWND *)((int)param_1 + 0x18),hdc);
    pvVar8 = (HGDIOBJ)FUN_005fc130(*(int *)((int)param_1 + 0x168));
    pvVar8 = SelectObject(hdc_00,pvVar8);
    local_90 = SelectPalette(hdc_00,DAT_006be5a0,0);
    pvVar9 = (HGDIOBJ)FUN_006002b0(4);
    pvVar9 = SelectObject(hdc_00,pvVar9);
    SetBkMode(hdc_00,1);
    SetTextColor(hdc_00,0xffffff);
    FUN_00615db0(&iStack_74,1);
    puStack_8 = (undefined1 *)0x4;
    pvVar10 = FUN_00616e10(auStack_68,*(int *)((int)param_1 + 0x198));
    FUN_00615f00(pvVar10,0);
    ptVar15 = &tStack_84;
    UVar17 = 0x20;
    iVar13 = -1;
    pCVar11 = (LPCSTR)FUN_00615680(*(int *)((int)aiStack_70 + *(int *)(iStack_74 + 4)));
    DrawTextA(hdc_00,pCVar11,iVar13,ptVar15,UVar17);
    FUN_00615660(*(void **)((int)aiStack_70 + *(int *)(iStack_74 + 4)),0);
    FUN_00616c30(&iStack_74,0);
    FUN_00616900(auStack_68,0);
    pvVar10 = FUN_00616e10(auStack_68,*(int *)((int)param_1 + 0x19c));
    FUN_00615f00(pvVar10,0);
    ptVar15 = &tStack_84;
    UVar17 = 0x22;
    iVar13 = -1;
    pCVar11 = (LPCSTR)FUN_00615680(*(int *)((int)aiStack_70 + *(int *)(iStack_74 + 4)));
    DrawTextA(hdc_00,pCVar11,iVar13,ptVar15,UVar17);
    FUN_00615660(*(void **)((int)aiStack_70 + *(int *)(iStack_74 + 4)),0);
    SelectObject(hdc_00,pvVar8);
    SelectObject(hdc_00,pvVar9);
    SelectPalette(hdc_00,local_90,0);
    DeleteDC(hdc_00);
    puStack_8 = (undefined1 *)0xffffffff;
    FUN_00615ea0((int)auStack_60);
    FUN_00615440(auStack_60);
  }
  ExceptionList = pvStack_10;
  return;
}

