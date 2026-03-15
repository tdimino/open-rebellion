
void __thiscall FUN_00414a80(void *this,undefined4 param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  int iVar3;
  HWND hWnd;
  HDC pHVar4;
  LPBYTE pBVar5;
  int iVar6;
  undefined4 uVar7;
  int iVar8;
  BOOL BVar9;
  HPALETTE pHVar10;
  void *pvVar11;
  int *piVar12;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062aa9c;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(int **)((int)this + 0xc4) != (int *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)(**(int **)((int)this + 0xc4) + 8))();
    DestroyWindow(*(HWND *)(*(int *)((int)this + 0xc4) + 0x18));
    if (*(undefined4 **)((int)this + 0xc4) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0xc4))(1);
    }
    *(undefined4 *)((int)this + 0xc4) = 0;
  }
  piVar12 = *(int **)((int)this + 0xb8);
  if (piVar12 != (int *)0x0) {
    if (*(int **)((int)this + 0xc0) == piVar12) {
      *(undefined4 *)((int)this + 0xc0) = 0;
    }
    (**(code **)(*piVar12 + 8))();
  }
  piVar12 = *(int **)((int)this + 0xbc);
  if (piVar12 != (int *)0x0) {
    if (*(int **)((int)this + 0xc0) == piVar12) {
      *(undefined4 *)((int)this + 0xc0) = 0;
    }
    (**(code **)(*piVar12 + 8))();
  }
  if (*(int **)((int)this + 0xc0) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0xc0) + 8))();
    iVar6 = *(int *)((int)this + 0xc0);
    if (*(int *)((int)this + 0xb8) == iVar6) {
      *(undefined4 *)((int)this + 0xb8) = 0;
    }
    if (*(int *)((int)this + 0xbc) == iVar6) {
      *(undefined4 *)((int)this + 0xbc) = 0;
    }
    PostMessageA(*(HWND *)((int)this + 0x18),0x469,0,iVar6);
    *(undefined4 *)((int)this + 0xc0) = 0;
  }
  pHVar4 = GetDC(*(HWND *)((int)this + 0x18));
  PatBlt(pHVar4,0,0,*(int *)((int)this + 0x38),*(int *)((int)this + 0x3c),0x42);
  FUN_00600230(pHVar4);
  ReleaseDC(*(HWND *)((int)this + 0x18),pHVar4);
  pBVar5 = FUN_00401440();
  iVar6 = FUN_005fefd0(1);
  if (iVar6 == 0) {
    FUN_005ff020((char *)pBVar5,s_textcomm_dll_006a7edc,1);
  }
  iVar6 = FUN_005fefd0(8);
  if (iVar6 == 0) {
    FUN_005ff020((char *)pBVar5,s_common_dll_006a7c98,8);
  }
  uVar7 = FUN_005fefd0(8);
  if (pBVar5 != (LPBYTE)0x0) {
    FUN_00618b60(pBVar5);
  }
  uVar1 = *(undefined4 *)((int)this + 0xa0);
  uVar2 = *(undefined4 *)((int)this + 0xa4);
  iVar6 = *(int *)((int)this + 0xa8);
  iVar3 = *(int *)((int)this + 0xac);
  switch(*(undefined4 *)((int)this + 0xdc)) {
  case 3:
    iVar8 = ShowCursor(1);
    while (iVar8 < 0) {
      iVar8 = ShowCursor(1);
    }
    FUN_00597340();
    FUN_00597360();
    iVar8 = *(int *)((int)this + 0xb8);
    piVar12 = (int *)0x0;
    if (iVar8 != 0) {
      if (*(int *)((int)this + 0xc0) == iVar8) {
        *(undefined4 *)((int)this + 0xc0) = 0;
      }
      BVar9 = DestroyWindow(*(HWND *)(iVar8 + 0x18));
      if ((BVar9 != 0) && (*(undefined4 **)((int)this + 0xb8) != (undefined4 *)0x0)) {
        (**(code **)**(undefined4 **)((int)this + 0xb8))(1);
      }
      *(undefined4 *)((int)this + 0xb8) = 0;
    }
    iVar8 = *(int *)((int)this + 0xbc);
    if (iVar8 != 0) {
      if (*(int *)((int)this + 0xc0) == iVar8) {
        *(undefined4 *)((int)this + 0xc0) = 0;
      }
      DestroyWindow(*(HWND *)(iVar8 + 0x18));
      if (*(undefined4 **)((int)this + 0xbc) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)this + 0xbc))(1);
      }
      *(undefined4 *)((int)this + 0xbc) = 0;
    }
    FUN_00597050();
    thunk_FUN_00409370();
    FUN_0041dae0(0);
    pvVar11 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (pvVar11 != (void *)0x0) {
      piVar12 = (int *)FUN_005fbd20(pvVar11,uVar7,0x4e21,2);
    }
    uStack_4 = 0xffffffff;
    if (piVar12 != (int *)0x0) {
      hWnd = *(HWND *)((int)this + 0x18);
      pHVar4 = GetDC(hWnd);
      pHVar10 = FUN_005fc3f0(piVar12);
      FUN_00600010(pHVar4,pHVar10);
      ReleaseDC(hWnd,pHVar4);
      FUN_005fbfa0(piVar12);
      FUN_00618b60((undefined *)piVar12);
    }
    pvVar11 = (void *)FUN_00618b70(0x108);
    uStack_4 = 1;
    if (pvVar11 == (void *)0x0) {
LAB_00414e2e:
      pvVar11 = (void *)0x0;
    }
    else {
      pvVar11 = FUN_00404c30(pvVar11,*(undefined4 *)((int)this + 0x1c),uVar1,uVar2,iVar6,iVar3,this)
      ;
    }
    break;
  case 4:
  case 5:
    pvVar11 = (void *)FUN_00618b70(0x194);
    uStack_4 = 2;
    if (pvVar11 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      pvVar11 = FUN_0040d7a0(pvVar11,*(undefined4 *)((int)this + 0x1c),uVar1,uVar2,iVar6,iVar3,
                             (int)this,1,8,0x2774,0xe,0,0,1,1);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)this + 0xc0) = pvVar11;
    *(undefined4 *)((int)this + 0x104) = 7;
  default:
    goto switchD_00414c2b_caseD_6;
  case 0xb:
  case 0xc:
    pvVar11 = (void *)FUN_00618b70(0x11c);
    uStack_4 = 3;
    if (pvVar11 == (void *)0x0) goto LAB_00414e2e;
    pvVar11 = FUN_00406950(pvVar11,*(undefined4 *)((int)this + 0x1c),uVar1,uVar2,iVar6,iVar3,this,
                           param_1);
  }
  uStack_4 = 0xffffffff;
  *(void **)((int)this + 0xc0) = pvVar11;
switchD_00414c2b_caseD_6:
  if (*(void **)((int)this + 0xc0) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)this + 0xc0),0);
    if (*(int **)((int)this + 0xc0) != (int *)0x0) {
      (**(code **)(**(int **)((int)this + 0xc0) + 4))(5);
    }
    if (*(int *)((int)this + 0xc0) != 0) {
      SetFocus(*(HWND *)(*(int *)((int)this + 0xc0) + 0x18));
    }
  }
  ExceptionList = pvStack_c;
  return;
}

