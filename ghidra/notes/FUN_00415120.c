
void __fastcall FUN_00415120(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int iVar4;
  HDC pHVar5;
  int *piVar6;
  HPALETTE pHVar7;
  void *pvVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062aadb;
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
    iVar1 = *(int *)(param_1 + 0xc0);
    if (*(int *)(param_1 + 0xb8) == iVar1) {
      *(undefined4 *)(param_1 + 0xb8) = 0;
    }
    if (*(int *)(param_1 + 0xbc) == iVar1) {
      *(undefined4 *)(param_1 + 0xbc) = 0;
    }
    PostMessageA(*(HWND *)(param_1 + 0x18),0x469,0,iVar1);
    *(undefined4 *)(param_1 + 0xc0) = 0;
  }
  pHVar5 = GetDC(*(HWND *)(param_1 + 0x18));
  PatBlt(pHVar5,0,0,*(int *)(param_1 + 0x38),*(int *)(param_1 + 0x3c),0x42);
  FUN_00600230(pHVar5);
  ReleaseDC(*(HWND *)(param_1 + 0x18),pHVar5);
  if (*(int *)(param_1 + 0xbc) == 0) {
    iVar1 = *(int *)(param_1 + 0xac);
    uVar2 = *(undefined4 *)(param_1 + 0xa0);
    uVar3 = *(undefined4 *)(param_1 + 0xa4);
    iVar4 = *(int *)(param_1 + 0xa8);
    if (*(int *)(param_1 + 0x108) != 0) {
      pvVar8 = (void *)FUN_00618b70(0x114);
      uStack_4 = 0;
      if (pvVar8 == (void *)0x0) {
        pvVar8 = (void *)0x0;
      }
      else {
        pvVar8 = FUN_0059a8d0(pvVar8,*(undefined4 *)(param_1 + 0x1c),uVar2,uVar3,iVar4,iVar1,param_1
                              ,&DAT_006b1460);
      }
      uStack_4 = 0xffffffff;
      *(void **)(param_1 + 0xbc) = pvVar8;
    }
    if (*(void **)(param_1 + 0xbc) != (void *)0x0) {
      FUN_005ffce0(*(void **)(param_1 + 0xbc),0);
      (**(code **)(**(int **)(param_1 + 0xbc) + 4))(5);
      SetFocus(*(HWND *)(*(int *)(param_1 + 0xbc) + 0x18));
    }
  }
  else {
    pHVar5 = GetDC(*(HWND *)(param_1 + 0x18));
    piVar6 = (int *)FUN_005ff440(*(int *)(param_1 + 0xbc));
    pHVar7 = FUN_005fc3f0(piVar6);
    FUN_00600010(pHVar5,pHVar7);
    (**(code **)(**(int **)(param_1 + 0xbc) + 4))(5);
    ReleaseDC(*(HWND *)(param_1 + 0x18),pHVar5);
  }
  *(undefined4 *)(param_1 + 0xc0) = *(undefined4 *)(param_1 + 0xbc);
  ExceptionList = pvStack_c;
  return;
}

