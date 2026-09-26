
void __fastcall FUN_00414eb0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  HDC pHVar4;
  int iVar5;
  int *piVar6;
  HPALETTE pHVar7;
  void *pvVar8;
  undefined4 uVar9;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062aabb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(int **)(param_1 + 0xc4) != (int *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)(**(int **)(param_1 + 0xc4) + 8))();
    DestroyWindow(*(HWND *)(*(int *)(param_1 + 0xc4) + 0x18));
    if (*(undefined4 **)(param_1 + 0xc4) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)(param_1 + 0xc4))(1);
    }
    *(undefined4 *)(param_1 + 0xc4) = 0;
  }
  if (*(int **)(param_1 + 0xc0) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0xc0) + 8))();
    iVar5 = *(int *)(param_1 + 0xc0);
    if (*(int *)(param_1 + 0xb8) == iVar5) {
      *(undefined4 *)(param_1 + 0xb8) = 0;
    }
    if (*(int *)(param_1 + 0xbc) == iVar5) {
      *(undefined4 *)(param_1 + 0xbc) = 0;
    }
    PostMessageA(*(HWND *)(param_1 + 0x18),0x469,0,iVar5);
    *(undefined4 *)(param_1 + 0xc0) = 0;
  }
  pHVar4 = GetDC(*(HWND *)(param_1 + 0x18));
  PatBlt(pHVar4,0,0,*(int *)(param_1 + 0x38),*(int *)(param_1 + 0x3c),0x42);
  FUN_00600230(pHVar4);
  ReleaseDC(*(HWND *)(param_1 + 0x18),pHVar4);
  iVar5 = ShowCursor(1);
  while (iVar5 < 0) {
    iVar5 = ShowCursor(1);
  }
  if (*(int *)(param_1 + 0xb8) == 0) {
    iVar5 = *(int *)(param_1 + 0xac);
    iVar1 = *(int *)(param_1 + 0xa8);
    uVar2 = *(undefined4 *)(param_1 + 0xa0);
    uVar3 = *(undefined4 *)(param_1 + 0xa4);
    if (*(int *)(param_1 + 0x108) != 0) {
      if ((*(int *)(param_1 + 0xe0) == 10) || (*(int *)(param_1 + 0xe0) == 9)) {
        uVar9 = 1;
      }
      else {
        uVar9 = 0;
      }
      pvVar8 = (void *)FUN_00618b70(0x49c);
      uStack_4 = 0;
      if (pvVar8 == (void *)0x0) {
        pvVar8 = (void *)0x0;
      }
      else {
        pvVar8 = FUN_00421c70(pvVar8,*(undefined4 *)(param_1 + 0x1c),uVar2,uVar3,iVar1,iVar5,param_1
                              ,*(undefined4 *)(param_1 + 0x108),uVar9);
      }
      uStack_4 = 0xffffffff;
      *(void **)(param_1 + 0xb8) = pvVar8;
    }
    if (*(void **)(param_1 + 0xb8) != (void *)0x0) {
      FUN_005ffce0(*(void **)(param_1 + 0xb8),0);
      (**(code **)(**(int **)(param_1 + 0xb8) + 4))(5);
      SetFocus(*(HWND *)(*(int *)(param_1 + 0xb8) + 0x18));
    }
  }
  else {
    pHVar4 = GetDC(*(HWND *)(param_1 + 0x18));
    piVar6 = (int *)FUN_005ff440(*(int *)(param_1 + 0xb8));
    pHVar7 = FUN_005fc3f0(piVar6);
    FUN_00600010(pHVar4,pHVar7);
    (**(code **)(**(int **)(param_1 + 0xb8) + 4))(5);
    ReleaseDC(*(HWND *)(param_1 + 0x18),pHVar4);
  }
  *(undefined4 *)(param_1 + 0xc0) = *(undefined4 *)(param_1 + 0xb8);
  ExceptionList = pvStack_c;
  return;
}

