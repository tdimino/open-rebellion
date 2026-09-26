
void __fastcall FUN_00605fc0(void *param_1)

{
  undefined4 *puVar1;
  LPCSTR lpchText;
  HDC hdc;
  int iVar2;
  void *pvVar3;
  uint uVar4;
  LONG LVar5;
  tagRECT tStack_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065652b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  lpchText = (LPCSTR)FUN_00583c40((int)param_1 + 0x98);
  hdc = GetDC(*(HWND *)((int)param_1 + 0x18));
  tStack_1c.left = 0;
  tStack_1c.top = 0;
  tStack_1c.right = 0;
  tStack_1c.bottom = 0;
  if (lpchText != (LPCSTR)0x0) {
    FUN_006002c0(param_1,hdc);
    DrawTextA(hdc,lpchText,-1,&tStack_1c,0xc24);
    LVar5 = tStack_1c.right;
    if (tStack_1c.right <= *(int *)((int)param_1 + 0x38)) {
      LVar5 = *(int *)((int)param_1 + 0x38);
    }
    iVar2 = FUN_005fc0e0(*(int **)((int)param_1 + 0xac));
    if (iVar2 < LVar5) {
      puVar1 = *(undefined4 **)((int)param_1 + 0xac);
      if (puVar1 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar1);
        FUN_00618b60((undefined *)puVar1);
      }
      pvVar3 = (void *)FUN_00618b70(0x28);
      uStack_4 = 0;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_005fbea0(pvVar3,(LVar5 + *(int *)((int)param_1 + 0xe0)) * 2,
                              (*(int *)((int)param_1 + 0x3c) + *(int *)((int)param_1 + 0xe0)) * 2,
                              *(uint **)((int)param_1 + 0xa8),hdc);
      }
      uStack_4 = 0xffffffff;
      *(void **)((int)param_1 + 0xac) = pvVar3;
    }
    FUN_006002f0(param_1,hdc);
    uVar4 = FUN_005f3070((int)param_1 + 0x98);
    *(uint *)((int)param_1 + 0xd0) = uVar4 & 0xffff;
  }
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),hdc);
  ExceptionList = pvStack_c;
  return;
}

