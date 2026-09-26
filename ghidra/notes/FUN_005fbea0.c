
void * __thiscall FUN_005fbea0(void *this,LONG param_1,LONG param_2,uint *param_3,HDC param_4)

{
  uint uVar1;
  BITMAPINFO *lpbmi;
  HBITMAP pHVar2;
  int iVar3;
  uint uVar4;
  DWORD *pDVar5;
  BITMAPINFO *pBVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655eeb;
  pvStack_c = ExceptionList;
  iVar3 = 0;
  ExceptionList = &pvStack_c;
  FUN_0060e5e0((void *)((int)this + 9),0,0,0);
  local_4 = 0;
  FUN_005fc100(param_3);
  uVar1 = FUN_005fc100(param_3);
  if ((int)uVar1 < 9) {
    iVar3 = param_3[4] << 2;
  }
  uVar4 = iVar3 + 0x28;
  FUN_005fc500(this,param_3[6],0x10);
  *(uint *)((int)this + 0x10) = param_3[4];
  lpbmi = (BITMAPINFO *)FUN_00618b70(uVar4);
  pBVar6 = lpbmi;
  for (uVar1 = uVar4 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    (pBVar6->bmiHeader).biSize = 0;
    pBVar6 = (BITMAPINFO *)&(pBVar6->bmiHeader).biWidth;
  }
  for (iVar3 = 0; iVar3 != 0; iVar3 = iVar3 + -1) {
    *(undefined1 *)&(pBVar6->bmiHeader).biSize = 0;
    pBVar6 = (BITMAPINFO *)((int)&(pBVar6->bmiHeader).biSize + 1);
  }
  pDVar5 = (DWORD *)*param_3;
  pBVar6 = lpbmi;
  for (uVar4 = uVar4 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    (pBVar6->bmiHeader).biSize = *pDVar5;
    pDVar5 = pDVar5 + 1;
    pBVar6 = (BITMAPINFO *)&(pBVar6->bmiHeader).biWidth;
  }
  for (iVar3 = 0; iVar3 != 0; iVar3 = iVar3 + -1) {
    *(char *)&(pBVar6->bmiHeader).biSize = (char)*pDVar5;
    pDVar5 = (DWORD *)((int)pDVar5 + 1);
    pBVar6 = (BITMAPINFO *)((int)&(pBVar6->bmiHeader).biSize + 1);
  }
  (lpbmi->bmiHeader).biWidth = param_1;
  (lpbmi->bmiHeader).biHeight = param_2;
  (lpbmi->bmiHeader).biSizeImage = 0;
  (lpbmi->bmiHeader).biCompression = 0;
  *(BITMAPINFO **)this = lpbmi;
  pHVar2 = CreateDIBSection(param_4,lpbmi,0,(void **)((int)this + 4),(HANDLE)0x0,0);
  *(HBITMAP *)((int)this + 0x1c) = pHVar2;
  GetLastError();
  if (*(void **)((int)this + 4) != (void *)0x0) {
    FUN_005fd170(this);
  }
  *(undefined4 *)((int)this + 0x24) = 0;
  ExceptionList = pvStack_c;
  return this;
}

