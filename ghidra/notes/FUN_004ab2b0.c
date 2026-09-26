
void __fastcall FUN_004ab2b0(void *param_1)

{
  HDC hdc;
  HDC pHVar1;
  void *this;
  undefined4 uVar2;
  int *this_00;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006383ab;
  pvStack_c = ExceptionList;
  this_00 = (int *)0x0;
  ExceptionList = &pvStack_c;
  SetWindowPos(*(HWND *)((int)param_1 + 0x18),(HWND)0x0,0,0,0,0,0x203);
  hdc = GetDC(*(HWND *)((int)param_1 + 0x18));
  pHVar1 = CreateCompatibleDC(hdc);
  *(HDC *)((int)param_1 + 0x128) = pHVar1;
  this = (void *)FUN_00618b70(0x28);
  uStack_4 = 0;
  if (this != (void *)0x0) {
    this_00 = FUN_005fbea0(this,*(LONG *)((int)param_1 + 0x38),*(LONG *)((int)param_1 + 0x3c),
                           *(uint **)((int)param_1 + 300),*(HDC *)((int)param_1 + 0x128));
  }
  uStack_4 = 0xffffffff;
  uVar2 = FUN_005fddd0(this_00,&LAB_004ac550);
  *(undefined4 *)((int)param_1 + 0x124) = uVar2;
  FUN_006075b0(param_1,this_00,1);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),hdc);
  ExceptionList = pvStack_c;
  return;
}

