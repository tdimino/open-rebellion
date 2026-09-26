
void __fastcall FUN_004070e0(void *param_1)

{
  HWND hWnd;
  void *this;
  undefined4 uVar1;
  HDC hDC;
  HPALETTE pHVar2;
  uint *puVar3;
  uint uVar4;
  undefined1 uVar5;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062990b;
  pvStack_c = ExceptionList;
  hWnd = *(HWND *)((int)param_1 + 0x18);
  ExceptionList = &pvStack_c;
  this = (void *)FUN_00618b70(0x28);
  puVar3 = (uint *)0x0;
  local_4 = 0;
  if (this != (void *)0x0) {
    uVar5 = 2;
    uVar4 = 0x4e22;
    uVar1 = FUN_006037f0(8);
    puVar3 = FUN_005fbd20(this,uVar1,uVar4,uVar5);
  }
  local_4 = 0xffffffff;
  hDC = GetDC(hWnd);
  pHVar2 = FUN_005fc3f0((int *)puVar3);
  FUN_00600010(hDC,pHVar2);
  ReleaseDC(hWnd,hDC);
  FUN_005ff2d0(param_1,puVar3,0,0);
  ExceptionList = pvStack_c;
  return;
}

