
void __thiscall FUN_00429200(void *this,int param_1,int param_2)

{
  HWND hWnd;
  int iVar1;
  HDC hDC;
  char *pcVar2;
  void *this_00;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c358;
  local_c = ExceptionList;
  this_00 = (void *)0x0;
  ExceptionList = &local_c;
  FUN_005f2f50(local_18);
  iVar1 = param_2;
  local_4 = 0;
  FUN_005f31a0(local_18,param_2);
  if (param_1 == 0x14c) {
    this_00 = (void *)((int)this + 0x178);
  }
  else if (param_1 == 0x14d) {
    this_00 = (void *)((int)this + 0x1ac);
  }
  else if ((param_1 == 0x14e) && (this_00 = (void *)((int)this + 0x1e0), 99999 < iVar1)) {
    param_2 = CONCAT22(DAT_0065d424,0x1302);
    FUN_005f3010(local_18,&param_2);
  }
  if (this_00 != (void *)0x0) {
    hWnd = *(HWND *)((int)this + 0x18);
    hDC = GetDC(hWnd);
    pcVar2 = (char *)FUN_00583c40((int)local_18);
    FUN_00601e20(this_00,hWnd,hDC,pcVar2);
    ReleaseDC(hWnd,hDC);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

