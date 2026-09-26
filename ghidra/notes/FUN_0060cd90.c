
void __fastcall FUN_0060cd90(void *param_1)

{
  int *this;
  HDC hdc;
  void *pvVar1;
  HDC hdc_00;
  HGDIOBJ h;
  undefined *puVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  HPALETTE pHVar6;
  undefined4 unaff_EBX;
  undefined4 unaff_EBP;
  int iVar7;
  code *pcVar8;
  HPALETTE unaff_EDI;
  int iStack_20;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656b6b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  hdc = GetDC(*(HWND *)((int)param_1 + 0x18));
  pHVar6 = DAT_006be5a0;
  pvVar1 = (void *)FUN_00618b70(0x28);
  uStack_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbea0(pvVar1,*(LONG *)((int)param_1 + 0x38),*(LONG *)((int)param_1 + 0x3c),
                          *(uint **)(*(int *)((int)param_1 + 0xa0) + 0xf0),hdc);
  }
  uStack_4 = 0xffffffff;
  *(void **)((int)param_1 + 0xa4) = pvVar1;
  GetStockObject(5);
  hdc_00 = CreateCompatibleDC(hdc);
  h = (HGDIOBJ)FUN_005fc130(*(int *)((int)param_1 + 0xa4));
  pcVar8 = SelectObject_exref;
  SelectObject(hdc_00,h);
  SelectPalette(hdc_00,pHVar6,0);
  SelectPalette(hdc,pHVar6,0);
  if (*(int *)((int)param_1 + 0xa4) != 0) {
    BitBlt(hdc_00,0,0,*(int *)((int)param_1 + 0x38),*(int *)((int)param_1 + 0x3c),hdc,0,0,0xcc0020);
    if (*(undefined **)((int)param_1 + 0xa8) == (undefined *)0x0) {
      this = *(int **)(*(int *)((int)param_1 + 0xa0) + 0xf4);
      if (this != (int *)0x0) {
        iVar4 = *(int *)((int)param_1 + 0x3c);
        iVar7 = 0;
        iStack_20 = 0;
        iVar3 = FUN_005fc0f0(this);
        pcVar8 = SelectObject_exref;
        if (iVar3 == iVar4) {
          if (0 < *(int *)((int)param_1 + 0x38)) {
            do {
              FUN_005fcc30(this,*(uint **)((int)param_1 + 0xa4),iVar7,0,0,0,(undefined4 *)0x0,0);
              iVar4 = FUN_005fc0e0(this);
              iVar7 = iVar7 + iVar4;
              pcVar8 = SelectObject_exref;
            } while (iVar7 < *(int *)((int)param_1 + 0x38));
          }
        }
        else if (0 < *(int *)((int)param_1 + 0x3c)) {
          do {
            FUN_005fcc30(this,*(uint **)((int)param_1 + 0xa4),0,iStack_20,0,0,(undefined4 *)0x0,0);
            iVar4 = FUN_005fc0f0(this);
            iStack_20 = iStack_20 + iVar4;
            pcVar8 = SelectObject_exref;
          } while (iStack_20 < *(int *)((int)param_1 + 0x3c));
        }
      }
    }
    else {
      puVar2 = (undefined *)
               FUN_005fddd0(*(void **)((int)param_1 + 0xa4),*(undefined **)((int)param_1 + 0xa8));
      FUN_005fe050(*(void **)((int)param_1 + 0xa4),0,0,(short)*(undefined4 *)((int)param_1 + 0x38),
                   (short)*(undefined4 *)((int)param_1 + 0x3c));
      FUN_005fe040(*(void **)((int)param_1 + 0xa4),0);
      FUN_00618b60(puVar2);
    }
    uVar5 = (*pcVar8)(hdc_00,*(undefined4 *)((int)param_1 + 0xac));
    *(undefined4 *)((int)param_1 + 0xac) = uVar5;
    pHVar6 = (HPALETTE)(*pcVar8)(hdc_00,unaff_EBX);
    Rectangle(hdc_00,0,0,*(int *)((int)param_1 + 0x38) + -1,*(int *)((int)param_1 + 0x3c) + -1);
    (*pcVar8)(hdc_00,pHVar6);
    (*pcVar8)(hdc_00,*(undefined4 *)((int)param_1 + 0xac));
    SelectPalette(hdc,unaff_EDI,0);
    SelectPalette(hdc_00,pHVar6,0);
    (*pcVar8)(hdc_00,unaff_EBP);
    DeleteDC(hdc_00);
    FUN_005ff2d0(param_1,*(uint **)((int)param_1 + 0xa4),0,0);
  }
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),hdc);
  ExceptionList = pvStack_c;
  return;
}

