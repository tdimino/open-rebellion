
void __fastcall FUN_0059ae70(void *param_1)

{
  HWND hWnd;
  void *this;
  uint *puVar1;
  undefined4 uVar2;
  HDC hdc;
  int iVar3;
  HPALETTE hpal;
  undefined1 *puVar4;
  undefined1 *puVar5;
  int iVar6;
  uint uVar7;
  undefined1 uVar8;
  tagPALETTEENTRY atStack_40c [10];
  undefined1 auStack_3e4 [984];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006518fe;
  pvStack_c = ExceptionList;
  hWnd = *(HWND *)((int)param_1 + 0x18);
  ExceptionList = &pvStack_c;
  this = (void *)FUN_00618b70(0x28);
  puVar1 = (uint *)0x0;
  local_4 = 0;
  if (this != (void *)0x0) {
    uVar8 = 2;
    uVar7 = 1000;
    uVar2 = FUN_006037f0(6);
    puVar1 = FUN_005fbd20(this,uVar2,uVar7,uVar8);
  }
  local_4 = 0xffffffff;
  hdc = GetDC(hWnd);
  iVar3 = FUN_005c14b0();
  hpal = FUN_005fc3f0((int *)puVar1);
  GetSystemPaletteEntries(hdc,0,0x100,atStack_40c);
  if (iVar3 != 0) {
    puVar5 = auStack_3e4 + 1;
    puVar4 = (undefined1 *)(iVar3 + 0x2a);
    iVar6 = 0xec;
    do {
      puVar4[(int)atStack_40c + (1 - iVar3)] = 4;
      puVar5[-1] = puVar4[-2];
      *puVar5 = puVar4[-1];
      puVar4[(int)atStack_40c - iVar3] = *puVar4;
      puVar5 = puVar5 + 4;
      puVar4 = puVar4 + 4;
      iVar6 = iVar6 + -1;
    } while (iVar6 != 0);
  }
  if (hpal != (HPALETTE)0x0) {
    SetPaletteEntries(hpal,0,0x100,atStack_40c);
  }
  FUN_005ff2d0(param_1,puVar1,0,0);
  FUN_00600010(hdc,hpal);
  ReleaseDC(hWnd,hdc);
  ExceptionList = pvStack_c;
  return;
}

