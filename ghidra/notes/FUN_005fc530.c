
undefined4 __thiscall FUN_005fc530(void *this,LPCSTR param_1)

{
  HFILE hFile;
  UINT UVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  short sStack_10;
  int iStack_e;
  
  if (*(undefined **)this != (undefined *)0x0) {
    FUN_00618b60(*(undefined **)this);
    *(undefined4 *)this = 0;
  }
  hFile = _lopen(param_1,0x20);
  if (hFile != -1) {
    UVar1 = _lread(hFile,&sStack_10,0xe);
    if (UVar1 != 0xe) {
      _lclose(hFile);
      return 0;
    }
    if (sStack_10 != DAT_006ac7a0) {
      _lclose(hFile);
      return 0;
    }
    uVar4 = iStack_e - 0xe;
    iVar2 = FUN_00618b70(uVar4);
    *(int *)this = iVar2;
    if (iVar2 == 0) {
      _lclose(hFile);
      return 0;
    }
    iVar2 = 0;
    for (; uVar4 != 0; uVar4 = uVar4 - uVar3) {
      uVar3 = 0x8000;
      if (uVar4 < 0x8001) {
        uVar3 = uVar4;
      }
      uVar3 = uVar3 & 0xffff;
      UVar1 = _lread(hFile,(LPVOID)(*(int *)this + iVar2),uVar3);
      if (uVar3 != UVar1) {
        _lclose(hFile);
        FUN_00618b60(*(undefined **)this);
        *(undefined4 *)this = 0;
        return 0;
      }
      iVar2 = iVar2 + uVar3;
    }
    _lclose(hFile);
    uVar4 = FUN_005fc120(this);
    if ((0xb < uVar4) && ((uVar4 < 0xd || (0xf < uVar4)))) {
      FUN_005fc4a0(this);
      FUN_005fc4d0(this);
      return 1;
    }
    FUN_00618b60(*(undefined **)this);
  }
  return 0;
}

